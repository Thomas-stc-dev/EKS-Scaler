from urllib.parse import parse_qs
import json
import boto3
import base64
import yaml
import re
from string import Template
from os import makedirs
from datetime import datetime, timedelta
from kubernetes import client, config
from botocore.signers import RequestSigner
from botocore.session import get_session
import os

TEMPLATE = """
apiVersion: v1
clusters:
- cluster:
    certificate-authority-data: $ca_auth_data
    server: $cluster_endpoint
  name: $cluster_arn
contexts:
- context:
    cluster: $cluster_arn
    user: $cluster_arn
  name: $cluster_arn
current-context: $cluster_arn
kind: Config
preferences: {}
users:
- name: $cluster_arn
  user:
    token: $token
"""

CLUSTER_TO_GROUP = {
    "wave-eks-istio-amd": "wa-node-group-1-20241115003535118900000001",
    "eks-simulation-hpa": "eks-node-group-hpa-1-2024082300130146580000000f",
    "eks-simulation-wa-ai": "eks-node-group-1-20241002045951652300000001"
    }


# eventbridge = boto3.client('events')

def get_eks_token(cluster_name, region, eks_client):
    
    session = get_session()
    
    # print(botocore.__version__)
    service_id = eks_client.meta.service_model.service_id

    
    # Assume the IAM role used by the Lambda function already has EKS access permissions
    signer = RequestSigner(
        service_id=service_id, #'eks',
        region_name=region,
        signing_name='sts',
        signature_version='v4',  # 올바른 서명 버전 지정
        credentials=boto3.Session().get_credentials(),
        event_emitter=session.get_component('event_emitter')  # Event Emitter 가져오기
        #endpoint_url=cluster_info['endpoint']
        #event_hooks=None  # event_hooks는 일반적으로 None으로 설정
    )

    # Create a signed URL for authentication
    params = {
        'method': 'GET',
        'url': 'https://sts.ap-northeast-1.amazonaws.com/?Action=GetCallerIdentity&Version=2011-06-15',
        'body': {},
        'headers': {'X-K8s-Aws-Id': cluster_name},
        'context': {}
    }

    signed_url = signer.generate_presigned_url(
        request_dict=params,
        operation_name='',
        expires_in=60
    )
    
    base64_url = base64.urlsafe_b64encode(signed_url.encode('utf-8')).decode('utf-8')
    
    return 'k8s-aws-v1.' + re.sub(r'=*', '', base64_url)
    
    
def update_yaml_file(template, output_file, variables):
    """
    YAML 템플릿 문자열을 사용자 지정 변수로 치환한 후, 결과를 YAML 파일로 저장합니다.
    :param template: 템플릿 문자열
    :param output_file: 출력될 YAML 파일 이름
    :param variables: 템플릿 문자열에 적용할 변수들의 딕셔너리
    """
    # 문자열 포맷팅으로 변수 치환
    formatted_template = Template(template).safe_substitute(variables)

    # YAML로 파싱하여 파일로 저장
    with open(output_file, "w", encoding="utf-8") as file:
        ctx = yaml.safe_load(formatted_template)
        yaml_data = yaml.dump(ctx)
        file.write(yaml_data)
    

def load_eks_config(cluster_name, region, cluster_info, eks_client):
    config_path = "/tmp/config"
    
    kube_config = {
        "ca_auth_data": cluster_info['certificateAuthority']['data'],
        "cluster_endpoint": cluster_info['endpoint'],
        "cluster_arn": cluster_info['arn'],
        "region": region,
        "token": get_eks_token(cluster_name, region, eks_client),
    }
    # print(kube_config)
    update_yaml_file(TEMPLATE, config_path, kube_config)
    
    config.load_kube_config(config_path)
    
def update_cluster_nodegroup(cluster_name, eks_client, scale_up):
    try:
        eks_response = eks_client.update_nodegroup_config(
            clusterName=cluster_name,
            nodegroupName=CLUSTER_TO_GROUP[cluster_name],
            scalingConfig={
                'minSize': 0,                            
                'maxSize': 10,
                'desiredSize': 2 if scale_up else 0
            }
     )
        print(f"EKS Node Group update response: {eks_response} and karpenter {"resumed" if scale_up else "paused"}")
        return True
    except Exception as e:
        print(f"Failed to update EKS Node Group and pause karpenter: {e}")
    

# Karpenter NodePool 재활성화
def set_cpu_limit_karpenter_nodepool(cluster_info, cluster_name, region, eks_client, cpu_limit):
    load_eks_config(cluster_name, region, cluster_info, eks_client)
    api_instance = client.CustomObjectsApi()
    nodepool_name = 'default'
    body = {
        "spec": {
            "limits": {
                "cpu": cpu_limit
            }
        }
    }
    try:
        api_instance.patch_cluster_custom_object(
            group='karpenter.sh',
            version='v1beta1',
            plural='nodepools',
            name=nodepool_name,
            body=body
        )
        print(f"Karpenter NodePool '{nodepool_name}' cpu set to {cpu_limit}.")
        return True
    except Exception as e:
        print(f"Failed to set cpu to {cpu_limit} -- Karpenter NodePool: {str(e)}")
        

def scale_karpenter_nodepool(cluster_info, cluster_name, region, cpu_limit, eks_client):
    try:
        
        scale_up = False if cpu_limit == 0 else True
        
        set_cpu_result = set_cpu_limit_karpenter_nodepool(cluster_info, cluster_name, region, eks_client, cpu_limit)
        update_cluster_nodegroup_result = update_cluster_nodegroup(cluster_name, eks_client, scale_up)
        
        if set_cpu_result and update_cluster_nodegroup_result:
            return {
                "statusCode": 200,
                "body": json.dumps({
                    "response_type": "ephemeral",
                    "text": f"Karpenter NodePool '{cluster_name}' cpu set to {cpu_limit}. Karpenter {"resumed" if scale_up else "paused"}."
                })
            }
    except Exception as e:
        print(f"Failed to Scale Karpenter NodePool: {str(e)}")
        return {
            "statusCode": 500,
            "body": json.dumps({
                "response_type": "ephemeral",
                "text": f"Failed to Scale {cpu_limit} -- Karpenter NodePool: {str(e)}"
            })
        }
        
def lambda_handler(event, context):
    try:
        params = event.get('queryStringParameters') or {}
        cpu_limit = params.get('cpu-limit') or None
        cluster_name = params.get('cluster-name') or None
        region = params.get('region') or 'ap-northeast-1'
        
        if cpu_limit is None:
            return {
                "statusCode": 400,
                "body": json.dumps({
                    "response_type": "ephemeral",
                    "text": "cpu-limit is required"
                })
            }
        if cluster_name is None:
            return {
                "statusCode": 400,
                "body": json.dumps({
                    "response_type": "ephemeral",
                    "text": "cluster-name is required"
                })
            }
        eks_client = boto3.client('eks', region_name=region)
        cluster_info = eks_client.describe_cluster(name=cluster_name)['cluster']
        return scale_karpenter_nodepool(cluster_info, cluster_name, region, int(cpu_limit), eks_client)
    except Exception as e:
        print(f"Error: {e}")
        print(str(e))
        return {
            "statusCode": 500,
            "body": json.dumps({
                "response_type": "ephemeral",
                "text": f"Internal server error: {str(e)}"
            })
        }