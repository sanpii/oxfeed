begin;

do $$
begin
    create role demo with nologin;
    exception when duplicate_object then
        raise notice 'not creating role demo -- it already exists';
end
$$;

insert into "user" (email, password, token)
    values ('demo', 'demo', uuid_generate_v4())
    on conflict (email) do nothing;

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
