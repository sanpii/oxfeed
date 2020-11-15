create extension if not exists "uuid-ossp";

create table source (
    source_id uuid primary key default uuid_generate_v4(),
    url text not null,
    title text not null,
    tags text[] not null,
    type text
);

create table item (
    entry_id uuid primary key default uuid_generate_v4(),
    source_id uuid references source(source_id) not null,
    link text not null,
    title text not null,
    content text not null,
    icon text,
    read_at timestamptz,
    created_at timestamptz not null default now()
);
