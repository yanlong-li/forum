#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cd "$SCRIPT_DIR"

if [ ! -f "backend/Cargo.toml" ]; then
    echo "Error: Backend not found"
    exit 1
fi

if [ ! -f "frontend/package.json" ]; then
    echo "Error: Frontend not found"
    exit 1
fi

echo "Starting Community Forum..."

if [ ! -d "$HOME/.cargo" ]; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

source "$HOME/.cargo/env"

echo "Building backend..."
cd backend
cargo build --release 2>&1 | tail -5

echo ""
echo "Starting backend server on port 3000..."
cargo run --release &
BACKEND_PID=$!

cd ../frontend

echo "Installing frontend dependencies..."
npm install 2>&1 | tail -3

echo ""
echo "Starting frontend dev server on port 5173..."
npm run dev &
FRONTEND_PID=$!

echo ""
echo "=============================================="
echo "Community Forum is running!"
echo "Backend API: http://localhost:3000"
echo "Frontend:    http://localhost:5173"
echo "=============================================="
echo ""
echo "Press Ctrl+C to stop all servers"

trap "kill $BACKEND_PID $FRONTEND_PID 2>/dev/null" EXIT

wait
