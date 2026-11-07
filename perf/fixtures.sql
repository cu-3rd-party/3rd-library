-- JSON-фикстуры для k6: id из золотой БД (клон БД сохраняет их). Выводится psql -At.
select json_build_object(
  'materialIds', (select json_agg(material_id) from (select material_id from material order by random() limit 3000) s),
  'authorIds',   (select json_agg(user_id) from (select user_id from web_user where email like 'author%@perf.test') s),
  'readerIds',   (select json_agg(user_id) from (select user_id from web_user where email like 'reader%@perf.test' order by random() limit 1000) s),
  -- пулы одноразовых заявок; author — номер автора (author{n}@perf.test)
  'rejected',    (select json_agg(json_build_object('id', s.submission_id, 'author', substring(u.email from 'author(\d+)@')::int))
                  from (select submission_id, user_id from submission where status = 'rejected' order by created_at, submission_id) s
                  join web_user u using (user_id)),
  'pending',     (select json_agg(submission_id) from (select submission_id from submission where status = 'pending_review' order by created_at, submission_id) s),
  'drafts',      (select json_agg(json_build_object('id', s.submission_id, 'author', substring(u.email from 'author(\d+)@')::int))
                  from (select submission_id, user_id from submission where status = 'draft' order by created_at, submission_id) s
                  join web_user u using (user_id)),
  -- сессии: n = номер reader; <= 6000 — для refresh, остальные — для logout
  'sessions',    (select json_agg(json_build_object('sid', s.session_id, 'userId', s.user_id, 'n', substring(u.email from 'reader(\d+)@')::int))
                  from session s join web_user u using (user_id) where u.email like 'reader%@perf.test'),
  'pools', json_build_object(
    'readers',  (select count(*) from web_user where email like 'reader%@perf.test'),
    'authors',  (select count(*) from web_user where email like 'author%@perf.test'),
    'verify',   (select count(*) from web_user where email like 'pending%@perf.test'),
    'resend',   (select count(*) from web_user where email like 'resend%@perf.test'))
);
