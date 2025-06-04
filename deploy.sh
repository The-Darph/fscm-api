#!/bin/bash

# === CONFIGURATION ===
REMOTE_USER="bill"
REMOTE_HOST="170.187.158.15"
REMOTE_DIR="/home/bill/fscm-api"
REPO_URL="/home/bill/git/fscm-api.git"
BRANCH="production"

# === DEPLOYMENT ===
echo "🚀 Connecting to $REMOTE_HOST..."

ssh "$REMOTE_USER@$REMOTE_HOST" bash <<EOF
  set -e

  echo "🔧 Creating app directory if it doesn't exist..."
  mkdir -p $REMOTE_DIR

  echo "📁 Changing to app directory..."
  cd $REMOTE_DIR

  if [ ! -d .git ]; then
    echo "🧬 Cloning repository..."
    git clone $REPO_URL .
  else
    echo "📡 Pulling latest changes from $BRANCH..."
    git reset --hard
    git clean -fd
    git pull origin $BRANCH
  fi

  echo "📦 Building in release mode..."
  export RUST_BACKTRACE=1
  export $(cat .env.production | xargs)
  /home/bill/.cargo/bin/cargo build -p cli --release

  echo "✅ Build complete. Binary is at: target/release/cli"

  echo "📂 Running migrations..."
  /home/bill/.cargo/bin/diesel migration run --migration-dir ./cli/migrations

  echo "🔁 Restarting systemd service..."
  echo "LunisTheG00dBoi!" | sudo -S systemctl restart fscm-api.service

  echo "🎉 Deployment to $REMOTE_HOST successful!"
EOF
