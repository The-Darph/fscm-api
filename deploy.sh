#!/bin/bash

set -e  # Exit on error

# === CONFIGURATION ===
APP_NAME="fscm-api"                     # Binary and service name
REMOTE_USER="youruser"                # VPS username
REMOTE_HOST="your.vps.ip"             # VPS IP or domain
REMOTE_DIR="/home/$REMOTE_USER/$APP_NAME"
SERVICE_NAME="$APP_NAME"              # systemd service name
ENV_FILE=".env.production"            # your local production .env

# === STEP 1: Build the app locally ===
echo "🔨 Building app in release mode..."
cargo build -p cli --release

# === STEP 2: Sync files to server ===
echo "📤 Syncing files to VPS..."
rsync -avz --delete \
    --exclude target/debug \
    --exclude .git \
    --exclude node_modules \
    --exclude '*.log' \
    ./target/release/cli \
    $ENV_FILE \
    cli/migrations \
    cli/diesel.toml \
    $REMOTE_USER@$REMOTE_HOST:$REMOTE_DIR/

# === STEP 3: Run migrations ===
echo "📦 Running database migrations..."
ssh $REMOTE_USER@$REMOTE_HOST "
    cd $REMOTE_DIR && \
    export \$(cat $ENV_FILE | xargs) && \
    diesel migration run --migration-dir ./cli/migrations
"

# === STEP 4: Restart systemd service ===
echo "🚀 Restarting service..."
ssh $REMOTE_USER@$REMOTE_HOST "
    sudo systemctl restart $SERVICE_NAME && \
    sudo systemctl status $SERVICE_NAME --no-pager
"

echo "✅ Deployment complete!"
