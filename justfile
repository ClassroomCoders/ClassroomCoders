compose := "docker compose -f deploy/compose.dev.yaml"
migrations := "crates/migrate/migrations"

# A local .env (gitignored) can override anything below.
set dotenv-load

export DATABASE_URL := env("DATABASE_URL", "postgres://classcoders:classcoders@127.0.0.1:5433/classcoders")

# Start infra, all services, and the web app
dev:
    process-compose up

# Start only NATS + Postgres in the background
infra:
    {{compose}} up -d --wait

# Stop infra containers
infra-down:
    {{compose}} down

# Stop infra and wipe NATS/Postgres data
infra-reset:
    {{compose}} down -v

# Apply pending migrations
migrate: infra
    cargo run -p migrate

# Create a new reversible migration: `just migrate-add create_users`
migrate-add name:
    sqlx migrate add -r --source {{migrations}} {{name}}

# Revert the most recent migration
migrate-revert:
    sqlx migrate revert --source {{migrations}}

# Drop, recreate, and re-migrate the database
db-reset: infra
    sqlx database reset -y --source {{migrations}}

# Refresh the offline query cache in .sqlx/ (commit it) for DB-less builds
db-prepare: migrate
    cargo sqlx prepare --workspace
