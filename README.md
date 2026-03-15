# Serverless HTTP Relay Proxy

A lightweight, high-performance HTTP relay proxy implemented in Rust and deployed as an AWS Lambda Function.

## Overview

This proxy allows you to forward HTTP requests to any target URL by specifying the destination in the request headers. It is designed to be used as a utility for serverless environments where outbound IP addresses are rotating or where a central proxy is needed for authentication management.

## API Documentation

The full API specification is available in the [openapi.yaml](./openapi.yaml) file. You can load this file into any OpenAPI/Swagger viewer (like [Swagger Editor](https://editor.swagger.io)) to explore the interface.

### Quick Usage

To use the proxy, send an HTTP request to your Lambda Function URL with the following custom headers:

| Header | Required | Description |
|--------|----------|-------------|
| `X-Proxy-Auth` | Yes | Your secret authentication token. |
| `X-Target-URL` | Yes | The full destination URL (e.g., `https://api.example.com/v1/resource`). |
| `X-Target-Method` | Yes | The HTTP method to use for the target (`GET`, `POST`, `PUT`, etc.). |

### Example (curl)

```bash
curl -X POST "https://your-proxy-url.on.aws/" \
     -H "X-Proxy-Auth: your-secret-here" \
     -H "X-Target-URL: https://httpbin.org/post" \
     -H "X-Target-Method: POST" \
     -H "Content-Type: application/json" \
     -d '{"key": "value"}'
```

## Security

Authentication is performed via the `X-Proxy-Auth` header. This value must match the `PROXY_AUTH_SECRET` environment variable configured on the Lambda function.

## Development

- **Language**: Rust
- **Runtime**: AWS Lambda (`provided.al2023`)
- **Infrastructure**: AWS CDK (TypeScript)

### Building

```bash
cargo lambda build --release --arm64
```

### Deployment

See the [infra/README.md](./infra/README.md) for detailed deployment instructions.
