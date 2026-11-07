-- Золотой набор данных. Все пользователи делят один пароль 'perf-password' (хеш берём у hash-donor).
-- Размеры — в таблице params; менять можно, но тогда пересобрать: ./golden.sh
create temp table params as select
  20000  as readers,        -- обычные верифицированные пользователи reader{n}@perf.test
  50     as authors,        -- авторы заявок author{n}@perf.test (под них логинится k6)
  100000 as materials,      -- опубликованные материалы
  6000   as pending_pool,   -- заявки pending_review: для очереди модерации и решений (расходуются)
  6000   as rejected_pool,  -- rejected: для PATCH (расходуются)
  2000   as draft_pool,
  6000   as verify_pool,    -- неподтверждённые pending{n}: код 123456 (расходуются)
  6000   as resend_pool;    -- неподтверждённые resend{n}, код выдан час назад (расходуются)

create temp table donor as select password_hash from web_user where email = 'hash-donor@perf.test';

insert into web_user (email, password_hash, name, bio, roles, is_email_verified)
select 'moderator' || n || '@perf.test', (select password_hash from donor), 'Moderator ' || n,
       'Moderator bio', array['user', 'moderator'], true
from generate_series(1, 3) n;

insert into web_user (email, password_hash, name, bio, is_email_verified)
select 'author' || n || '@perf.test', (select password_hash from donor), 'Author ' || n,
       'Author bio ' || repeat('lorem ipsum ', 10), true
from params, generate_series(1, params.authors) n;

insert into web_user (email, password_hash, name, bio, is_email_verified)
select 'reader' || n || '@perf.test', (select password_hash from donor), 'Reader ' || n, 'bio ' || n, true
from params, generate_series(1, params.readers) n;

insert into web_user (email, password_hash, name, is_email_verified, verification_code,
                      verification_code_issued_at, verification_code_expires_at)
select 'pending' || n || '@perf.test', (select password_hash from donor), 'Pending ' || n, false, '123456',
       now(), now() + interval '30 days'
from params, generate_series(1, params.verify_pool) n;

insert into web_user (email, password_hash, name, is_email_verified, verification_code,
                      verification_code_issued_at, verification_code_expires_at)
select 'resend' || n || '@perf.test', (select password_hash from donor), 'Resend ' || n, false, '654321',
       now() - interval '1 hour', now() + interval '30 days'
from params, generate_series(1, params.resend_pool) n;

-- Материалы: авторы — authors + часть readers, чтобы у профилей была разная «толщина».
create temp table author_ids as
  select user_id, row_number() over (order by email) as rn from web_user
  where email like 'author%@perf.test';

create temp table words as select * from unnest(array[
  'algebra','calculus','physics','chemistry','history','networks','compilers','databases','statistics',
  'mechanics','optics','linguistics','economics','genetics','robotics','security','graphs','logic']) with ordinality as w(word, ord);

insert into material (user_id, title, description, courses, subjects, type, difficulty, published_at)
select (select user_id from author_ids where rn = 1 + (n % (select authors from params))),
       (select word from words where ord = 1 + (n % 18)) || ' ' ||
       (select word from words where ord = 1 + ((n / 18) % 18)) || ' notes #' || n,
       'Description of material ' || n || '. ' || repeat('Lorem ipsum dolor sit amet. ', 4),
       array[(1 + n % 4)::text],
       array[(select word from words where ord = 1 + (n % 18))],
       (array['lecture', 'seminar', 'exam', 'notes', 'lab'])[1 + n % 5],
       (array['easy', 'medium', 'hard'])[1 + n % 3],
       now() - (n || ' minutes')::interval
from params, generate_series(1, params.materials) n;

-- По одному файлу на материал (+ второй у каждого третьего).
insert into material_file (user_id, name, size_bytes, extension, mime_type, storage_key)
select m.user_id, 'material-' || row_number() over () || '.pdf', 100000 + (random() * 5000000)::int, 'pdf',
       'application/pdf', 'seed/' || m.material_id || '/' || k
from material m, generate_series(1, 2) k
where k = 1 or (hashtext(m.material_id::text) % 3 = 0);

-- привязка: файлы seed/<material_id>/<k>
insert into material_file_rel (material_id, file_id)
select split_part(storage_key, '/', 2)::uuid, file_id from material_file where storage_key like 'seed/%';

-- Заявки. approved — по каждому пятому материалу (даёт реалистичные счётчики модерации).
insert into submission (user_id, material_id, title, description, courses, subjects, type, difficulty, status,
                        moderator_id, published_at, submitted_at, reviewed_at)
select m.user_id, m.material_id, m.title, m.description, m.courses, m.subjects, m.type, m.difficulty, 'approved',
       (select user_id from web_user where email = 'moderator1@perf.test'), m.published_at, m.created_at,
       m.published_at
from material m where hashtext(m.material_id::text) % 5 = 0;

create temp table sub_src (kind text, n int);
insert into sub_src select 'pending_review', n from params, generate_series(1, params.pending_pool) n;
insert into sub_src select 'rejected', n from params, generate_series(1, params.rejected_pool) n;
insert into sub_src select 'draft', n from params, generate_series(1, params.draft_pool) n;

insert into submission (user_id, title, description, courses, subjects, type, difficulty, status,
                        moderator_id, moderator_comment, submitted_at, reviewed_at)
select (select user_id from author_ids where rn = 1 + (s.n % (select authors from params))),
       s.kind || ' submission #' || s.n, 'Seeded ' || s.kind || ' submission. ' || repeat('text ', 20),
       array[(1 + s.n % 4)::text], array['algebra'], 'lecture', 'medium', s.kind,
       case s.kind when 'rejected' then (select user_id from web_user where email = 'moderator1@perf.test') end,
       case s.kind when 'rejected' then 'needs work' else '' end,
       case s.kind when 'draft' then null else now() - (s.n || ' seconds')::interval end,
       case s.kind when 'rejected' then now() - (s.n || ' seconds')::interval end
from sub_src s;

-- Поиск `ilike '%…%'` и список по published_at должны работать на «прогретой» статистике.
analyze;

-- Готовые сессии (id сессии = refresh-токен) для эндпоинтов, которые расходуют сессию: refresh и logout.
-- readers 1..6000 → refresh, 6001..12000 → logout.
insert into session (session_id, user_id, verified, expires_at)
select uuid_generate_v4(), u.user_id, true, now() + interval '30 days'
from web_user u
where u.email ~ '^reader([1-9]|[1-9][0-9]{1,3}|1[0-2][0-9]{3}|12000)@perf.test$' and substring(u.email from 'reader(\d+)@')::int <= 12000;
