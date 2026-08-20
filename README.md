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
src/
main.rs        Entry point
handlers.rs    Request handlers
routes.rs      Route definitions
models.rs      Data models
database.rs    Database access
state.rs       Shared application state
lib.rs
migrations/      Schema migrations
tests/           Tests (including injection scenarios)
docs/            Supporting notes

## Setup

1. Install a recent Rust toolchain (`rustup`).
2. Copy the example environment file:
   ```bash
   cp .env.example .env

## Details
Edit .env with real values if required (never commit .env).

## Build and run
cargo build
cargo run

## Run tests
cargo test

## Security Focus 
Commits in this repository intentionally show both a vulnerable search path and the fixed version. The goal is to make the difference concrete: how injection occurs, and how parameterised queries prevent it.

## Status
Work in progress. Suitable for learning and demonstration of secure backend patterns in Rust.


---

### 2. agripay  
**Status:** Already has a strong README. Only light tightening recommended.

You can leave the existing README as-is. If you want a slightly cleaner top section, replace the opening lines with:

```markdown
# AgriPay

Loan cash today. Repay after harvest.

A working MVP for smallholder farmers who need input financing before harvest. Farmers submit crop and harvest details; an explainable scoring engine produces a loan offer; approved funds are disbursed to M-Pesa via PayHero. Repayment is collected the same way with an STK Push at harvest time.

**Stack:** React (Vite) · Express · PayHero · Node.js
