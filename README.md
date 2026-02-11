# REST SMTP API

A lightweight REST API that sends emails via SMTP. Built with Rust using [warp](https://github.com/seanmonstar/warp) and [lettre](https://github.com/lettre/lettre).

## Quick Start

### 1. Create a config file

Create `api_config.json` mapping API tokens to SMTP servers:

```json
{
  "api_keys": {
    "your-secret-api-token": {
      "host": "smtp.example.com",
      "credentials": {
        "username": "user@example.com",
        "password": "your-password"
      }
    }
  }
}
```

You can define multiple API tokens, each with its own SMTP backend. The server connects via STARTTLS on port 587.

### 2. Run the server

```bash
cargo build --release
API_CONFIG_FILE=./api_config.json ./target/release/rest-smtp-api
```

The server starts on `0.0.0.0:9002`.

## Configuration

| Environment Variable | Default | Description |
|---|---|---|
| `API_CONFIG_FILE` | `./api_config.json` | Path to the JSON config file |
| `RUST_LOG` | `rest_smtp=info` | Log level ([env_logger](https://docs.rs/env_logger) syntax) |

## API

### `GET /health`

Returns `200 OK`. Use this for health checks.

### `POST /send`

Sends an email. Requires authentication via the `x-api-token` header.

**Headers:**

| Header | Required | Description |
|---|---|---|
| `Content-Type` | Yes | Must be `application/json` |
| `x-api-token` | Yes | API token matching a key in your config |

**Body:**

```json
{
  "from": "sender@example.com",
  "reply_to": "reply@example.com",
  "to": "recipient@example.com",
  "subject": "Hello",
  "body": "Email body text",
  "attachments": [
    {
      "filename": "document.pdf",
      "content": "<base64-encoded-bytes>",
      "mime_type": "application/pdf"
    }
  ]
}
```

| Field | Type | Required | Description |
|---|---|---|---|
| `from` | string | Yes | Sender email address |
| `reply_to` | string | Yes | Reply-to email address |
| `to` | string | Yes | Recipient email address |
| `subject` | string | Yes | Email subject line |
| `body` | string | Yes | Plain text email body |
| `attachments` | array | No | File attachments (see below) |

**Attachment object:**

| Field | Type | Required | Description |
|---|---|---|---|
| `filename` | string | Yes | Filename shown to recipient |
| `content` | string | Yes | File content, base64-encoded |
| `mime_type` | string | Yes | MIME type (e.g. `application/pdf`, `image/png`) |

**Responses:**

| Status | Description |
|---|---|
| `201` | Email sent successfully |
| `400` | Invalid request (bad email address, malformed JSON, invalid base64) |
| `401` | Invalid or missing API token |
| `500` | SMTP transport error |

### Examples

**Plain text email:**

```bash
curl -X POST http://localhost:9002/send \
  -H "Content-Type: application/json" \
  -H "x-api-token: your-secret-api-token" \
  -d '{
    "from": "sender@example.com",
    "reply_to": "sender@example.com",
    "to": "recipient@example.com",
    "subject": "Hello",
    "body": "This is a plain text email."
  }'
```

**Email with attachment:**

```bash
curl -X POST http://localhost:9002/send \
  -H "Content-Type: application/json" \
  -H "x-api-token: your-secret-api-token" \
  -d '{
    "from": "sender@example.com",
    "reply_to": "sender@example.com",
    "to": "recipient@example.com",
    "subject": "Invoice attached",
    "body": "Please find the invoice attached.",
    "attachments": [
      {
        "filename": "invoice.pdf",
        "content": "'$(base64 -w0 invoice.pdf)'",
        "mime_type": "application/pdf"
      }
    ]
  }'
```

## Docker

```bash
docker build -t rest-smtp-api .
docker run -p 9002:9002 -v ./api_config.json:/rest-smtp-api/api_config.json rest-smtp-api
```

The image uses a multi-stage build (Rust build stage + slim Debian runtime) and does not depend on OpenSSL (TLS is handled by `rustls`).

## Technical Details

- **SMTP transport:** STARTTLS on port 587
- **TLS:** rustls (no OpenSSL dependency)
- **Request body limit:** 16 MB
- **Async runtime:** Tokio
