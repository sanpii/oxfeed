create extension if not exists "uuid-ossp";
create extension if not exists pgcrypto;

create table source (
    source_id uuid primary key default uuid_generate_v4(),
    user_id uuid references "user"(user_id) not null,
    url text not null unique,
    title text not null,
    tags text[] not null,
    last_error text,

    unique(source_id, user_id)
);

create index source_user_id on source(user_id);

create table item (
    item_id uuid primary key default uuid_generate_v4(),
    source_id uuid references source(source_id) not null,
    id text not null,
    icon text,
    link text not null unique,
    title text not null,
    content text,
    read bool default false,
    favorite bool default false,
    published timestamptz not null default now(),

    unique(source_id, id)
);

create index item_read on item(read);
create index item_favorite on item(favorite);
create index item_source_id on item(source_id);

create table "user" (
    user_id uuid primary key default uuid_generate_v4(),
    name text not null,
    email text not null,
    password text not null,
    token uuid
);

create index user_read on "user"(token);
