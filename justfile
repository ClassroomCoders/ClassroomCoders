compose := "docker compose -f deploy/compose.dev.yaml"

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
