# vote-svc

> **Note:** This project's purpose is for learning the Rust programming language.

A simple voting service REST API built with Rust, following clean architecture principles.

## Overview

`vote-svc` is an HTTP service that manages election candidates. It demonstrates how to structure a Rust web service using layered architecture (delivery → usecase → repository → domain).

## Tech Stack

| Dependency | Purpose |
|---|---|
| [actix-web](https://actix.rs/) | HTTP server framework |
| [sqlx](https://github.com/launchbadge/sqlx) | Async PostgreSQL driver |
| [tokio](https://tokio.rs/) | Async runtime |
| [serde](https://serde.rs/) | JSON serialization/deserialization |
| [chrono](https://github.com/chronotope/chrono) | Date/time handling |
| [uuid](https://github.com/uuid-rs/uuid) | UUID types |
| [mockall](https://github.com/asomers/mockall) | Mock generation for unit tests |
| [config](https://github.com/mehcode/config-rs) | Configuration management |
| [dotenvy](https://github.com/allan2/dotenvy) | `.env` file loading |
| [anyhow](https://github.com/dtolnay/anyhow) | Flexible error handling |
| [thiserror](https://github.com/dtolnay/thiserror) | Structured error types |

## Project Structure

```
src/
├── main.rs                        # Entry point — wires up server, database, and handlers
├── lib.rs
├── app.rs                         # Application-level shared state (AppHandlerData)
├── config.rs
├── candidate/                     # Candidate feature module
│   ├── domain/                    # Core domain: entities, errors, repository trait
│   ├── usecase/                   # Business logic (get candidates)
│   ├── repository/                # PostgreSQL repository implementation
│   └── delivery/http/             # HTTP handlers and route definitions
├── infrastructure/
│   ├── config/                    # App configuration (loaded from environment)
│   ├── database/                  # PostgreSQL connection pool management
│   └── http/                      # HTTP server setup
└── utils/                         # Shared response utilities
```

## API Endpoints

| Method | Path | Description |
|---|---|---|
| `GET` | `/candidates` | List all candidates |

### Response Format

All endpoints return a unified JSON response envelope:

**Success**
```json
{
  "status": true,
  "message": "Successfully processed candidate",
  "error_code": "",
  "data": { ... }
}
```

**Error**
```json
{
  "status": false,
  "message": "failed process data",
  "error_code": "-1",
  "data": null
}
```

## Configuration

Configuration is loaded from environment variables. Copy `.env.example` to `.env` and fill in the values:

```bash
cp .env.example .env
```

`.env.example`:

```env
APP__ENV=production
APP__NAME=EVOTE-SERVICE

SERVER__ADDR=0.0.0.0
SERVER__PORT=8080

DATABASE__ADDR=localhost
DATABASE__PORT=5432
DATABASE__NAME=vote
DATABASE__USERNAME=your_username
DATABASE__PASSWORD=your_password
DATABASE__MAX_CONN=2
DATABASE__MIN_CONN=1
```

Environment variables use double underscores (`__`) as a separator for nested config keys.

## Getting Started

### Prerequisites

- Rust (edition 2024)
- PostgreSQL

### Run

```bash
# Copy and fill in environment config
cp .env.example .env

# Run the service
cargo run
```

The server will start on the address and port defined in your `.env` (default: `0.0.0.0:8080`).

### Run Tests

```bash
cargo test
```

## Graceful Shutdown

The server listens for `Ctrl+C` and shuts down cleanly — stopping the HTTP server and closing the database connection pool before exiting.
