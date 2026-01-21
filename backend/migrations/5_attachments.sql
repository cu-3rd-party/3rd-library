create table if not exists attachment (
	article_id UUID references article (article_id) on delete set null,
	attachment_id UUID primary key default uuid_generate_v1mc(),
	file_name text not null,
	file_path text not null,

	created_at timestamptz not null default now(),
	updated_at timestamptz not null default now()
);

select trigger_updated_at('attachment');

create index attachment_article_id_idx on attachment (article_id);
