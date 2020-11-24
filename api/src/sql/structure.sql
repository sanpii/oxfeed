create extension if not exists "uuid-ossp";

create table source (
    source_id uuid primary key default uuid_generate_v4(),
    url text not null unique,
    title text not null,
    tags text[] not null,
    last_error text
);

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
