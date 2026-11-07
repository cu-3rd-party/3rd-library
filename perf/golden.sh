#!/usr/bin/env bash
# Собирает «золотую» БД: миграции + большой набор данных (seed.sql) → шаблон $GOLDEN_DB.
# Запускать один раз (или при смене схемы/seed.sql); каждый прогон run.sh клонирует шаблон.
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

trap stop_backend EXIT
infra_up
cargo build --release -p library-web --manifest-path "$ROOT_DIR/Cargo.toml"

psql -qd postgres -c "update pg_database set datistemplate=false where datname='$GOLDEN_DB'" 2>/dev/null || true
psql -qd postgres -c "drop database if exists $GOLDEN_DB" -c "create database $GOLDEN_DB"

# Бэкенд сам накатывает миграции; заодно регистрируем одного пользователя, чтобы
# получить настоящий argon2-хеш пароля (его получат все seed-пользователи).
start_backend "$GOLDEN_DB"
curl -fs -X POST "http://$BACKEND_ADDR/api/auth/register" -H 'content-type: application/json' \
  -d '{"name":"hash donor","email":"hash-donor@perf.test","password":"perf-password"}' >/dev/null
stop_backend

echo "seeding…"
psql -q -v ON_ERROR_STOP=1 -d "$GOLDEN_DB" -f "$PERF_DIR/seed.sql"
psql -q -d "$GOLDEN_DB" -c "vacuum analyze"
psql -q -d "$GOLDEN_DB" -At -f "$PERF_DIR/fixtures.sql" >"$PERF_DIR/k6/fixtures.json"
psql -qd postgres -c "alter database $GOLDEN_DB is_template true"
psql -d "$GOLDEN_DB" -c "select 'users' t, count(*) from web_user union all select 'materials', count(*) from material union all select 'submissions', count(*) from submission union all select 'files', count(*) from material_file"
echo "golden DB '$GOLDEN_DB' готова, фикстуры: perf/k6/fixtures.json"
