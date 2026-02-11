# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A lightweight REST API that sends emails via SMTP. Built with Rust using warp (HTTP) and lettre (SMTP). Deployed via Docker (Coolify).

## Commands

- **Build:** `cargo build --release`
- **Run:** `cargo run` (listens on `0.0.0.0:9002`)
- **Check:** `cargo check`
- **Docker:** `docker build -t rest-smtp-api .`

No tests exist in this project.

## Architecture

Four source files in `src/`:

- **main.rs** — Loads config from `API_CONFIG_FILE` env var (default: `./api_config.json`), initializes logging, starts warp server on port 9002.
- **filters.rs** — Warp filter composition. Defines two routes: `POST /send` (authenticated email sending) and `GET /health`.
- **routes.rs** — Route handlers. `send_mail` validates the request, looks up SMTP config by API token, sends the email.
- **mailer.rs** — SMTP transport layer using lettre. Defines `MailOptions`, `SmtpConnectionOptions`, error types, and the async `send_mail_smtp` function using STARTTLS.

## Config Format

The API config JSON maps API tokens to SMTP connection details:

```json
{
  "api_keys": {
    "some-api-token": {
      "host": "smtp.example.com",
      "credentials": { "username": "user", "password": "pass" }
    }
  }
}
```

## API

- **POST /send** — Header `x-api-token` for auth. JSON body: `{ from, reply_to, to, subject, body }`. Returns 201 on success.
- **GET /health** — Returns 200 OK.

## Key Details

- TLS: Uses `rustls` (no OpenSSL dependency) via lettre's `tokio1-rustls-tls` feature.
- Body size limit: 16 MB.
- Logging: `RUST_LOG` env var, defaults to `rest_smtp=info`. Uses `pretty_env_logger`.
