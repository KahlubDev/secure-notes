# Secure Notes

Rust-based notes service focused on practical application security.

The project implements a notes API with database-backed storage and deliberately explores common vulnerabilities (including SQL injection) and their remediation. It is intended as a working example of secure coding practices rather than a production product.

## Stack

- Rust
- SQLite (via migrations)
- Structured handlers, models, and routes

## Features

- Create and search notes through an API
- Database migrations for schema management
- Demonstration of a SQL injection vulnerability in note search
- Remediated, parameterised query version of the same endpoint
- Basic test coverage under `tests/`
- Environment-based configuration (see `.env.example`)

## Project structure
