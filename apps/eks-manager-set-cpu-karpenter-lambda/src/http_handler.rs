use anyhow::anyhow;
use k8s_openapi::{api::authentication::v1::UserInfo, serde_json::{self}};
use kube::{
    client::Client as KubeClient,
    config::{KubeConfigOptions, Kubeconfig, AuthInfo},  // Added AuthInfo import
    Config,
};
use kube::{
    api::{Api, DynamicObject, Patch, PatchParams},
    discovery,
};
use lambda_http::{Body, Error, Request, RequestExt, Response};
use tracing::error;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    config_generator();
    let api_group_name = "karpenter.sh";
    // get cpu_limit from event and default it to 1000 i64 type
    let cpu_limit = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("cpu-limit"))
        .unwrap_or("-1")
        .parse::<i64>()
        .unwrap();
    if cpu_limit < 0 {
        return failure_response("cpu-limit param is required", 400);
    }
    // get context from event and default it to empty string
    let ctx = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("context"))
        .unwrap_or("")
        .to_string();
    if ctx.is_empty() {
        return failure_response("context param is required", 400);
    }
    let cpu_limit_setter = set_cpu_limit(cpu_limit, ctx).await;
    if cpu_limit_setter.is_err() {
        let err = cpu_limit_setter.err().unwrap();
        error!("Failed to set cpu limit - {:?}", err);
        return failure_response("Failed to set cpu limit", 500);
    }
    let current_directory = std::env::current_dir().unwrap().display();

    // Return a success response
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(
            format!(
                "Successfully changed cpu limit to {} for the group {}",
                cpu_limit, api_group_name
            )
            .into(),
        )
        .map_err(Box::new)?;

    Ok(resp)
}

async fn set_cpu_limit(cpu_limit: i64, ctx:String) -> anyhow::Result<()> {
    // create a custom config with cluster_name and region

    // let client = KubeClient::try_default().await?;
    // create kubeconfig with clustername as ekf
    // Load kubeconfig with specific cluster context
    // set the server url 




    let kubeconfig_result = Config::from_kubeconfig(&KubeConfigOptions {
        context: Some(ctx),
        ..Default::default()
    })
    .await;

    let kubeconfig = match kubeconfig_result {
        Ok(config) => config,
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to load kubeconfig: {}", e));
        }
    };
    
    let client_result = KubeClient::try_from(kubeconfig);



    let client = match client_result {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create Kubernetes client: {}", e);
            return
             Err(anyhow::anyhow!("Failed to create Kubernetes client: {}", e));
        }
    };

    let api_group_name = "karpenter.sh";
    // API Group
    let apigroup = discovery::group(&client, api_group_name)
        .await
        .map_err(|e| anyhow!("Karpenter.sh Failed to get api group - {:?}", e))?;
    // Find by kind
    let recommended_kind = apigroup.recommended_kind("NodePool");
    let Some(api_resource) = recommended_kind else {
        error!("NodePool Failed to get api resource");
        return Err(anyhow::anyhow!("NodePool Failed to get api resource"));
    };
    println!("NodePool api resource: {:?}", api_resource);

    // DynamicObject Api 생성
    let dynamic_object: Api<DynamicObject> =
        Api::namespaced_with(client.clone(), "", &api_resource.0);
    let patch_params = PatchParams::default();
    let json_patch = serde_json::json!({
        "spec": {
            "limits": {
                "cpu": cpu_limit
            }
        }
    });
    let patched_object = dynamic_object
        .patch("default", &patch_params, &Patch::Merge(&json_patch))
        .await
        .map_err(|e| anyhow!("[set_cpu_limit] Failed to set cpu limit - {:?}", e))?;
    let object_data = patched_object.data;
    let new_cpu_limit = object_data["spec"]["limits"]["cpu"].as_i64().unwrap();
    assert_eq!(new_cpu_limit, cpu_limit);
    Ok(())
}

fn failure_response(message: &str, status: u16) -> Result<Response<Body>, Error> {
    return Ok(Response::builder()
        .status(status)
        .body(message.into())
        .unwrap());
}

// function to read yaml file
fn read_yaml_file(file_path: &str) -> Result<String, Error> {
    let file = std::fs::read_to_string(file_path).map_err(|e| {
        error!("Failed to read file: {}", e);
        Error::from(e)
    })?;
    Ok(file)
}



#[derive(Serialize)]
struct Cluster {
    #[serde(rename = "certificate-authority-data")]
    certificate_authority_data: String,
    server: String,
}

#[derive(Serialize)]
struct Context {
    cluster: String,
    user: String,
}

#[derive(Serialize)]
struct User {
    #[serde(rename = "client-certificate-data")]
    client_certificate_data: String,
    #[serde(rename = "client-key-data")]
    client_key_data: String,
}

#[derive(Serialize)]
struct KubeConfig {
    apiVersion: String,
    kind: String,
    clusters: Vec<NamedCluster>,
    contexts: Vec<NamedContext>,
    users: Vec<NamedUser>,
    #[serde(rename = "current-context")]
    current_context: String,
}

#[derive(Serialize)]
struct NamedCluster {
    name: String,
    cluster: Cluster,
}

#[derive(Serialize)]
struct NamedContext {
    name: String,
    context: Context,
}

#[derive(Serialize)]
struct NamedUser {
    name: String,
    user: User,
}

pub fn generate_kubeconfig(
    cluster_name: &str,
    server: &str,
    ca_data: &str,
    user_name: &str,
    client_cert_data: &str,
    client_key_data: &str,
    context_name: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let kubeconfig = KubeConfig {
        apiVersion: "v1".to_string(),
        kind: "Config".to_string(),
        clusters: vec![NamedCluster {
            name: cluster_name.to_string(),
            cluster: Cluster {
                certificate_authority_data: ca_data.to_string(),
                server: server.to_string(),
            },
        }],
        contexts: vec![NamedContext {
            name: context_name.to_string(),
            context: Context {
                cluster: cluster_name.to_string(),
                user: user_name.to_string(),
            },
        }],
        users: vec![NamedUser {
            name: user_name.to_string(),
            user: User {
                client_certificate_data: client_cert_data.to_string(),
                client_key_data: client_key_data.to_string(),
            },
        }],
        current_context: context_name.to_string(),
    };

    let yaml = serde_yaml::to_string(&kubeconfig)?;
    let mut file = File::create(output_path)?;
    file.write_all(yaml.as_bytes())?;

    Ok(())
}

pub fn config_generator() {
    let result = generate_kubeconfig(
        "arn:aws:eks:ap-northeast-1:935103045726:cluster/wave-eks-istio-amd",
        "https://30B5A808990BCDE99FE965488501AABB.gr7.ap-northeast-1.eks.amazonaws.com",
        "LS0tLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0tCk1JSURCVENDQWUyZ0F3SUJBZ0lJWDhXYjcrRjlMM3d3RFFZSktvWklodmNOQVFFTEJRQXdGVEVUTUJFR0ExVUUKQXhNS2EzVmlaWEp1WlhSbGN6QWVGdzB5TkRBME1qY3dOREU0TkRCYUZ3MHpOREEwTWpVd05ESXpOREJhTUJVeApFekFSQmdOVkJBTVRDbXQxWW1WeWJtVjBaWE13Z2dFaU1BMEdDU3FHU0liM0RRRUJBUVVBQTRJQkR3QXdnZ0VLCkFvSUJBUUMrZmc1bGUwTDU3bGNWazA0czlrT1hMWXJzQzNVVjBEOUphQnJuemtFV1hzTGNKSDBhMlpOU0xSeGgKZGlNdXBSNGlkQWVTYS9zL1ZyYlAzc3pRZkxETHZaT2dJb3RHcWNYeDNCMi82ZVVueWF1eXJGL1V4SlJ1U0VmVgpUUVozcWRpRFNHenlvaUkzRi9nQldOTHZhL2FSNk50RmZPTlR0Z1BEVUhkRDFRUjFBYmZUSFd5R2djZTJkMzBwCjB2TTU1RENyVTJLWXhhaDlPNEFGNGNLMkJ4Sk9vVEJTU1kvN0doV3FkLzNEamZDbHVnVityV3JMM1pqd3NRYzQKN1FyNTdIMmpTVkt2dXgvQkIxaGZyQXRBZ0tHMWZxUUUyYmJrTHEzWVhWZ04zckRrSWJISWZkQm0wSVdzcDJjSgp0WDRyUXU0dFJCdktzcWhtdGIxTGczcDI1ZG5kQWdNQkFBR2pXVEJYTUE0R0ExVWREd0VCL3dRRUF3SUNwREFQCkJnTlZIUk1CQWY4RUJUQURBUUgvTUIwR0ExVWREZ1FXQkJSNnN1bEVMdi9qc0k1Z1REekY4RkhrM2ZBMy96QVYKQmdOVkhSRUVEakFNZ2dwcmRXSmxjbTVsZEdWek1BMEdDU3FHU0liM0RRRUJDd1VBQTRJQkFRQzEybHFhemk1NApBZWhyb3NsQlJnZTNrVEhZVnRsUUNOcjdFTmRaQlRxYjU0bXNyamlZUDJ5eWhzZE02MUpjYmRidGpwaUUxYk53CitQZGIzQzE3T2dYZ0ROdVpPVWhEbzMyYi9sQWMwV3VDdVY1enhhV0QyeTcvTmJmYkxWbTJ3dm0yR0hGY1dlbEkKRDZaZVBqYnFZZE9LYXhTY2NHSnB6SVo0SHYvS2liMG9mZFUzYlMrSXFEaGNjTXpqM2cvaGJYVXdWaC90b1BQQwpLVXM1R2ZaclQwMEhkV2xFKzBqcExRejFUUlVrbEtWdlA1K2hmTjdhdkloK0pZUHM0WnVIb2JqRlY1bWJ4L2pBCmFWTVU5aWxkeHR2Mk5NbkRxUXdpMHdUSlZ1UjMrM1Nqb0lZdDgrVXBacUw3UEpmamZpVnFjOURMSEJvTVFqNTMKMnVJQlJvN0xwM1Y2Ci0tLS0tRU5EIENFUlRJRklDQVRFLS0tLS0K",
        "arn:aws:eks:ap-northeast-1:935103045726:cluster/wave-eks-istio-amd",
        "base64-client-cert",
        "base64-client-key",
        "arn:aws:eks:ap-northeast-1:935103045726:cluster/wave-eks-istio-amd",
        "kubeconfig.yaml",
    );

    match result {
        Ok(_) => println!("Kubeconfig generated successfully!"),
        Err(e) => eprintln!("Error generating kubeconfig: {}", e),
    }
}
