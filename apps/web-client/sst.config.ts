// eslint-disable-next-line @typescript-eslint/triple-slash-reference
/// <reference path="./.sst/platform/config.d.ts" />

import { Linkable } from "./.sst/platform/src/components";

export default $config({
  app(input) {
    return {
      name: "eks-scaler-web-client",
      // removal: input?.stage === "production" ? "retain" : "remove",
      removal: "remove",
      // protect: ["production"].includes(input?.stage),
      home: "aws",
      
    };
  },
  async run() {
   
    const Table = new sst.aws.Dynamo("EKS-Scaler-Table", {
      fields: {
        id: "string"
      },
      primaryIndex: { hashKey: "id" }
    });
    const SetKarpenterCpuLambda = await aws.lambda.getFunction({functionName: "eks-scaler-SetKarpenterCPUFunction-wdLYETT9Cz1N"})
    const EksTerminateEc2Lambda = await aws.lambda.getFunction({functionName: "eks-scaler-ec2"})

    const LambdaKarpenter = new sst.Linkable("KarpenterCPU", {
      properties: {
        ...SetKarpenterCpuLambda
      },
      include: [
        sst.aws.permission({
          actions: ["lambda:*"],
          resources: [SetKarpenterCpuLambda.arn]
        })
      ]
    });
    const LambdaEc2 = new sst.Linkable("TerminateEc2", {
      properties: {
        ...EksTerminateEc2Lambda
      },
      include: [
        sst.aws.permission({
          actions: ["lambda:*"],
          resources: [EksTerminateEc2Lambda.arn]
        })
      ]
    });

    new sst.aws.Nextjs("eksScalerWebClient", {
      link: [Table, LambdaKarpenter, LambdaEc2],

    });
  },
});
