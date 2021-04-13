begin;

create extension if not exists "uuid-ossp";
create extension if not exists pgcrypto;

create table if not exists "user" (
    user_id uuid primary key default uuid_generate_v4(),
    email text not null unique,
    password text not null,
    token uuid
);

create index if not exists user_read on "user"(token);

create table if not exists source (
    source_id uuid primary key default uuid_generate_v4(),
    user_id uuid references "user"(user_id) not null,
    url text not null,
    title text not null,
    tags text[] not null,
    last_error text,
    active bool not null,
    webhooks uuid[],

    unique(url, user_id)
);

create index if not exists source_user_id on source(user_id);

create table if not exists webhook (
    webhook_id uuid primary key default uuid_generate_v4(),
    user_id uuid references "user"(user_id) not null,
    name text not null,
    url text not null,
    last_error text,
    mark_read bool not null default false,

    unique(name, user_id)
);

create index if not exists webhook_user_id on source(user_id);

create table if not exists item (
    item_id uuid primary key default uuid_generate_v4(),
    source_id uuid references source(source_id) not null,
    id text not null,
    icon text,
    link text not null,
    title text not null,
    content text,
    read bool default false,
    favorite bool default false,
    published timestamptz not null default now(),

    unique(source_id, id),
    unique(source_id, link)
);

create index if not exists item_read on item(read);
create index if not exists item_favorite on item(favorite);
create index if not exists item_source_id on item(source_id);

create or replace function crypt_password()
    returns trigger
    language plpgsql
as $$
begin
    if new.password != old.password
    then
        new.password := crypt(new.password, gen_salt('bf'));
    end if;
    return new;
end;
$$;

drop trigger if exists crypt_user_password on "user";
create trigger crypt_user_password
    before insert or update on "user"
    for each row
    execute function crypt_password();

create or replace function notify_new()
    returns trigger
    language plpgsql
as $$
begin
    perform pg_notify('item_new', token::text)
        from "user"
        join source on source_id = new.source_id;

    return new;
end;
$$;

drop trigger if exists notify_new_item on item;
create trigger notify_new_item
    after insert on item
    for each row
    execute function notify_new();

create schema if not exists fts;

create materialized view if not exists fts.item as
    select item.item_id,
        setweight(to_tsvector(coalesce(item.title, '')), 'A')
        || setweight(to_tsvector(coalesce(item.content, '')), 'B') as document
        from item;

create index fts_item_item_id on fts.item(item_id);
create index fts_item_document on fts.item using gin(document);

commit;
