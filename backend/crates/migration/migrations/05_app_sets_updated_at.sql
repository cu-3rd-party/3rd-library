-- `updated_at` ставит сам бэкенд: состояние в его памяти должно совпадать с БД побайтно,
-- а триггер подменял метку временем транзакции (с групповым коммитом — временем пачки).
drop trigger if exists set_updated_at on web_user;
drop trigger if exists set_updated_at on material;
drop trigger if exists set_updated_at on submission;
-- у material_file нет колонки updated_at: триггер ломал бы любой update этой таблицы
drop trigger if exists set_updated_at on material_file;
