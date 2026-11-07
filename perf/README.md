# Нагрузочное тестирование (k6)

Замер задержек **по каждому эндпоинту** бэкенда на настоящей инфраструктуре (PostgreSQL 16 + S3 RustFS в Docker,
бэкенд — release-сборка) и на «золотой» предзаполненной БД.

```bash
./golden.sh                       # один раз: миграции + seed (~32k юзеров, 100k материалов, 34k заявок) → шаблон БД
./run.sh                          # mixed: взвешенный реалистичный трафик, RATE=100 rps, 20s разгон + 2m
SCENARIO=isolated ./run.sh        # каждый эндпоинт отдельно, по очереди (ISO_DURATION=30 с каждый)
ONLY=materials_search,login SCENARIO=isolated ./run.sh
RATE=300 DURATION=5m ./run.sh
```

- **Golden DB.** `golden.sh` создаёт БД `library_golden` и помечает её шаблоном. Каждый `run.sh` делает
  `create database … template library_golden` — прогон всегда стартует с одинакового состояния, а запись
  (регистрации, заявки, решения модерации) не накапливается между запусками. Размеры данных — в начале `seed.sql`.
  Пароль всех seed-пользователей — `perf-password`.
- **Реальная инфраструктура.** `docker-compose.yml` (проект `lib-perf`, порты 55432/59000, не мешает dev-стеку).
  Rate limit бэкенда выключен (`RATE_LIMIT_CAPACITY=0`), иначе k6 с одного IP упрётся в 429.
- **Одноразовые данные.** Эндпоинты, расходующие состояние (verify-email, resend, refresh, logout, PATCH заявки,
  решение модерации), берут записи из пулов seed (6000 шт.). При исчерпании пула метрика `pool_exhausted > 0` и
  предупреждение в отчёте — увеличьте пулы в `seed.sql` и пересоберите `golden.sh`.
- **Отчёт.** Таблица avg/p50/p90/p95/p99/max по эндпоинтам, бюджеты p95 (`p95` в таблице `E` в `k6/scenario.js`)
  работают как k6-thresholds. Файлы — в `results/<время>/`: `summary.txt`, `summary.json`, `backend-metrics.txt`
  (серверные гистограммы с `:9091/metrics` для сверки).
- **Новый эндпоинт** — запись в таблице `E` в `k6/scenario.js` (вес для mixed, RPS для isolated, бюджет).
