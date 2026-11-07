-- Индексы под медленные эндпоинты (найдены нагрузочным тестом perf/).

-- Подстроковый поиск `ilike '%…%'` по названию материала и имени пользователя (GET /api/materials?search=, GET /api/users?search=).
create extension if not exists pg_trgm;
create index if not exists idx_material_title_trgm on material using gin (title gin_trgm_ops);
create index if not exists idx_web_user_name_trgm on web_user using gin (name gin_trgm_ops);

-- Лента опубликованных материалов: `order by published_at desc, material_id` + count(*) без обхода таблицы.
create index if not exists idx_material_feed on material (published_at desc, material_id) where published_at is not null;

-- Список авторов (GET /api/users) и материалы автора: join/агрегация по user_id только по опубликованным.
create index if not exists idx_material_author_published on material (user_id) where published_at is not null;
-- Сортировка авторов по имени.
create index if not exists idx_web_user_name on web_user (name, user_id);

-- Очередь модерации и «мои заявки»: фильтр по статусу + сортировка по created_at; счётчики статусов — index-only scan.
create index if not exists idx_submission_status_created on submission (status, created_at, submission_id);
create index if not exists idx_submission_user_created on submission (user_id, created_at, submission_id);
