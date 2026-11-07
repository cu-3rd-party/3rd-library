# Общие настройки perf-скриптов (source-им, не запускаем).
PERF_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$PERF_DIR")"
COMPOSE=(docker compose -f "$PERF_DIR/docker-compose.yml")

export PGHOST=localhost PGPORT=55432 PGUSER=perf PGPASSWORD=perf
GOLDEN_DB=${GOLDEN_DB:-library_golden}
BACKEND_ADDR=${BACKEND_ADDR:-127.0.0.1:18080}
METRICS_ADDR=${METRICS_ADDR:-127.0.0.1:19091}
BACKEND_BIN="$ROOT_DIR/target/release/library-web"
STATE_DIR="$PERF_DIR/.state"
mkdir -p "$STATE_DIR"

infra_up() {
  "${COMPOSE[@]}" up -d --wait db s3
}

# $1 — имя БД
start_backend() {
  local db=$1
  env DATABASE_URL="postgres://perf:perf@localhost:55432/$db" \
    S3_ENDPOINT=http://localhost:59000 S3_ACCESS_KEY=perf S3_SECRET_KEY=perf-secret-key \
    LISTEN_ADDR="$BACKEND_ADDR" METRICS_ADDR="$METRICS_ADDR" \
    HMAC_KEY=perf-hmac-key RUST_LOG="${RUST_LOG:-warn}" \
    RATE_LIMIT_CAPACITY=0 RATE_LIMIT_REFILL_PER_SEC=0 OPENAPI_ENABLED=false \
    "$BACKEND_BIN" >"$STATE_DIR/backend.log" 2>&1 &
  echo $! >"$STATE_DIR/backend.pid"
  for _ in $(seq 1 100); do
    curl -fs "http://$BACKEND_ADDR/api/health" >/dev/null 2>&1 && return 0
    sleep 0.3
  done
  echo "backend не поднялся, см. $STATE_DIR/backend.log" >&2
  return 1
}

stop_backend() {
  [[ -f "$STATE_DIR/backend.pid" ]] || return 0
  kill "$(cat "$STATE_DIR/backend.pid")" 2>/dev/null || true
  wait "$(cat "$STATE_DIR/backend.pid")" 2>/dev/null || true
  rm -f "$STATE_DIR/backend.pid"
}
