import * as cdk from 'aws-cdk-lib/core';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import { Construct } from 'constructs';
import * as path from 'path';

export class InfraStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // Task 5.2: Define Lambda function resource pointing to the arm64 binary.
    // The binary is built with `cargo lambda build --release --arm64`
    // and placed in target/lambda/serverless-proxy/bootstrap
    const proxyFn = new lambda.Function(this, 'ServerlessProxyFn', {
      functionName: 'serverless-proxy',
      runtime: lambda.Runtime.PROVIDED_AL2023,
      architecture: lambda.Architecture.ARM_64,
      handler: 'bootstrap',
      code: lambda.Code.fromAsset(
        path.join(__dirname, '..', '..', 'target', 'lambda', 'serverless-proxy')
      ),
      memorySize: 128,
      timeout: cdk.Duration.seconds(30),
      environment: {
        // Task 5.4: PROXY_AUTH_SECRET placeholder — set via AWS console,
        // Secrets Manager, or `cdk deploy --parameters` after initial deploy.
        PROXY_AUTH_SECRET: 'CHANGE_ME',
      },
      description: 'HTTP relay proxy — forwards requests to arbitrary targets with header-based auth',
    });

    // Task 5.3: Configure Function URL with NONE auth mode (auth handled in-Lambda).
    const fnUrl = proxyFn.addFunctionUrl({
      authType: lambda.FunctionUrlAuthType.NONE,
    });

    // Output the Function URL for easy access after deploy.
    new cdk.CfnOutput(this, 'ProxyFunctionUrl', {
      value: fnUrl.url,
      description: 'URL of the HTTP relay proxy Lambda function',
    });
  }
}
