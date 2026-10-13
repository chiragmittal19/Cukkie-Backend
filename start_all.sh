#!/usr/bin/env bash
set -e

export PATH="$HOME/.cargo/bin:$PATH"

echo "====================================================="
echo "   Starting Cukkie Backend Microservices (Rust)      "
echo "====================================================="
echo "  - User Service:   http://127.0.0.1:8081"
echo "  - Recipe Service: http://127.0.0.1:8082"
echo "  - Log Service:    http://127.0.0.1:8083"
echo "====================================================="

cargo run -p user_service &
PID_USER=$!

cargo run -p recipe_service &
PID_RECIPE=$!

cargo run -p log_service &
PID_LOG=$!

cleanup() {
    echo ""
    echo "Shutting down Cukkie services..."
    kill $PID_USER $PID_RECIPE $PID_LOG 2>/dev/null || true
}

trap cleanup INT TERM EXIT
wait
