#!/bin/bash

# If .env exists,then back up first
if [ -f .env ]; then
    cp .env .env.bak
    echo "🔁 Existing .env backed up as .env.bak"
fi

echo "🔧 Generating .env..."

cat > .env <<EOF
# Application Config
APP_NAME=vito
RUST_LOG=info
HOST=0.0.0.0
PORT=8000

# Database Config
DATABASE_URL=postgres://dbuser:dbpassword@127.0.0.1:5432/dockerdjango
DATABASE_NAME=dockerdjango
DATABASE_USERNAME=dbuser
DATABASE_PASSWORD=dbpassword
DATABASE_HOST=db
DATABASE_PORT=5432

# RabbitMQ Config
RABBITMQ_URL=amqp://guest:guest@rabbitmq:5672/
RABBITMQ_DEFAULT_USER=guest
RABBITMQ_DEFAULT_PASS=guest

# Redis Config
REDIS_URL=redis://127.0.0.1:6379

# Secret Key
SECRET_KEY=vito
EOF

echo ".env file generated successfully."
