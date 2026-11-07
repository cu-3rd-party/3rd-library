# Library

Библиотека учебных материалов cu3rd: Rust REST API (`backend/`, axum + sqlx, слоистая архитектура
как в [template-rust-svelte](https://github.com/pxc1984/template-rust-svelte)) и React SPA (`frontend/`).

```
Cargo.toml   Cargo workspace (members = backend/crates/*), общие версии зависимостей и линты
backend/     крейты domain / application / infrastructure / migration / test-support / web — см. backend/README.md
frontend/    React + Vite, раздаётся nginx (порт 8080, без root)
```

## Запуск

```bash
# всё в Docker: фронт :5173, API :8080 (Swagger UI — /api/docs/swagger-ui), Grafana :3000
for f in env/*.example; do cp -n "$f" "${f%.example}"; done
docker compose up -d --build

# локально
docker compose up -d db s3
DATABASE_URL=postgres://backend:backend@localhost:5432/db cargo run -p library-web
```

## Проверки

```bash
make backend-lint     # fmt + clippy
make backend-test     # все тесты; интеграционным нужен Docker или TEST_DATABASE_URL
```

## Деплой

Через [self-service](https://git.cu3rd.ru/cu3rd/self-service) (проект `library`, приложения `backend` и `frontend`
на одном домене: `/api` — бэкенд, остальное — фронт). Воркфлоу — `.forgejo/workflows/ci.yml`:

| Окружение | Домен | Когда |
|---|---|---|
| dev | https://dev.lib.cu3rd.ru | каждый пуш в `master` |
| prod | https://lib.cu3rd.ru | пуш тега на коммит из `master` или Actions → CI and deploy → Run workflow на `master` (поле `ref`: тег/коммит, пусто — текущий master) |

self-service принимает доставки только из запусков на `master`, поэтому пуш тега сам ничего не выкатывает:
он проверяет, что тег на `master`, и запускает тот же воркфлоу на `master` с `ref = тег`.

Секреты репозитория (Settings → Actions): `SELF_SERVICE_DELIVERY_TOKEN` (выдаёт платформа), `DEV_HMAC_KEY`,
`PROD_HMAC_KEY` (ключи подписи токенов), `SMTP_PASSWORD`; переменные: `SMTP_HOST`, `SMTP_USER`.
`REGISTRY_USER`/`REGISTRY_TOKEN` — общие для организации cu3rd. База (`DATABASE_URL`) и S3 (`S3_*`) выдаются
платформой.
