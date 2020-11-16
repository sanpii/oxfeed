create extension if not exists "uuid-ossp";

create table source (
    source_id uuid primary key default uuid_generate_v4(),
    url text not null unique,
    title text,
    icon text,
    tags text[] not null,
    type text
);

create table item (
    entry_id uuid primary key default uuid_generate_v4(),
    source_id uuid references source(source_id) not null,
    link text not null,
    title text not null,
    content text,
    read bool default false,
    published timestamptz not null default now()
);
