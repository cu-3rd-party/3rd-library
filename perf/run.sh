#!/usr/bin/env bash
# Прогон k6: клон золотой БД → бэкенд (release) на настоящих Postgres и S3 → k6 → отчёт.
#   ./run.sh                       # mixed: смешанный профиль нагрузки
#   SCENARIO=isolated ./run.sh     # каждый эндпоинт отдельно, по очереди
#   RATE=200 DURATION=2m ./run.sh  # (mixed) целевой RPS и длительность
# Аргументы после ./run.sh передаются в k6 как есть.
# Остальные параметры — переменные окружения k6/scenario.js (см. шапку файла).
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

RUN_DB="library_run_$(date +%s)"
RESULTS="$PERF_DIR/results/$(date +%Y%m%d-%H%M%S)"
mkdir -p "$RESULTS"

cleanup() {
  stop_backend
  psql -qd postgres -c "drop database if exists $RUN_DB with (force)" 2>/dev/null || true
}
trap cleanup EXIT

infra_up
[[ -x "$BACKEND_BIN" ]] || cargo build --release -p library-web --manifest-path "$ROOT_DIR/Cargo.toml"
[[ -f "$PERF_DIR/k6/fixtures.json" ]] || { echo "нет золотой БД — сначала ./golden.sh" >&2; exit 1; }

psql -qd postgres -c "create database $RUN_DB template $GOLDEN_DB"
start_backend "$RUN_DB"

set +e
k6 run \
  -e BASE_URL="http://$BACKEND_ADDR" \
  -e RESULTS_DIR="$RESULTS" \
  "$@" \
  "$PERF_DIR/k6/scenario.js"
status=$?
set -e

# Серверные гистограммы (то, что видит сам бэкенд) — для сверки с клиентскими замерами k6.
curl -fs "http://$METRICS_ADDR/metrics" >"$RESULTS/backend-metrics.txt" || true
echo "результаты: $RESULTS"
exit $status
