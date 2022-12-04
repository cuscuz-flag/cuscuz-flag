create extension if not exists "uuid-ossp";

create schema if not exists auth;

create table if not exists auth.accounts (
    id uuid default gen_random_uuid() not null,
    email varchar unique not null,
    password varchar not null,
    created_at timestamp without time zone default timezone('utc'::text, now()) not null,
    updated_at timestamp without time zone default timezone('utc'::text, now()) not null
);