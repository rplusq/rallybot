# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

RallyBot is a WhatsApp bot backend for managing padel (sports) sessions. It's built with Rust using clean architecture principles with a clear separation between core business logic and API infrastructure.

## Build and Development Commands

```bash
# Start PostgreSQL database (required for PostgreSQL storage)
docker-compose up -d

# Run the API server with in-memory storage
cargo run --bin rallybot-api

# Run with PostgreSQL storage
DATABASE_URL=postgresql://rallybot:rallybot@localhost:5432/rallybot cargo run --bin rallybot-api

# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p rallybot-core
cargo test -p rallybot-api

# Run a specific test
cargo test test_name

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Architecture

The codebase follows clean architecture with two main crates:

### rallybot-core

- **models.rs**: Domain models (User, Session, Venue, Registration)
- **repository/**: Repository traits defining data access contracts
- **services/**: Business logic services (UserService, SessionService, etc.)
- **storage/**: Storage implementations
  - `postgres.rs`: PostgreSQL implementation using SQLx
  - `in_memory.rs`: In-memory implementation for testing

### rallybot-api

- **main.rs**: Application entry point and server configuration
- **handlers/**: HTTP request handlers for each resource
- **state.rs**: Application state management using Arc<Repository>

## Key Design Patterns

1. **Repository Pattern**: All data access goes through repository traits, allowing easy switching between storage backends
2. **Dependency Injection**: Services receive repositories through constructor injection
3. **Shared State**: Uses Arc<Repository> for thread-safe shared state in the web server
4. **Error Handling**: Custom error types with proper error propagation

## Database

PostgreSQL is used for production storage. Migrations are in the `migrations/` directory.

```bash
# Run migrations (requires DATABASE_URL env var)
sqlx migrate run

# Create a new migration
sqlx migrate add migration_name
```

## Testing Strategy

- Unit tests are colocated with the code (in `#[cfg(test)]` modules)
- Integration tests use the in-memory storage implementation
- Tests follow the pattern: arrange, act, assert

## Environment Configuration

Copy `.env.example` to `.env` and configure:

- `DATABASE_URL`: PostgreSQL connection string
- `PORT`: Server port (default: 8080)
- `RUST_LOG`: Logging level

## WhatsApp Bot Functionality

The system is designed to support a WhatsApp bot (see WHATSAPP_BOT_SPEC.md) with features:

- Browse sessions by type (Coaching, Social, League, Mixed)
- Register for sessions
- View upcoming sessions
- Join waitlists
- User profile management with skill levels and preferences
