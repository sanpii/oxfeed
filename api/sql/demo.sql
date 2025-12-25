begin;

do $$
begin
    create role demo with nologin;
    exception when duplicate_object then
        raise notice 'not creating role demo -- it already exists';
end
$$;

drop trigger if exists crypt_user_password on "user";

insert into "user" (user_id, email, password, token)
    values ('041de296-c518-4005-a068-5989cebfddff', 'demo', crypt(new.password, gen_salt('bf')), uuid_generate_v4())
    on conflict (user_id) do nothing;

insert into source (user_id, url, title, tags, active) values
    ('041de296-c518-4005-a068-5989cebfddff', 'https://blog.rust-lang.org/feed.xml', 'Rust blog' '{rust}', true),
    ('041de296-c518-4005-a068-5989cebfddff', 'https://rustacean-station.org/podcast.rss', 'Rustacean Station' '{rust,podcast}', true);

create or replace function restore_token()
    returns trigger
    language plpgsql
as $$
begin
    new.token = old.token;
    return new;
end;
$$;

drop trigger if exists restore_user_token on "user";
create trigger restore_user_token
    before update on "user"
    for each row
    execute function restore_token();

alter materialized view fts.item owner to demo;
grant select on all tables in schema public to demo;
grant select on all tables in schema fts to demo;
grant usage on schema fts to demo;
grant insert, update on item to demo;
grant update(token) on "user" to demo;

commit;
