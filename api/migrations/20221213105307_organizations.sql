create schema if not exists orgs;

create table if not exists orgs.organizations (
    id uuid default gen_random_uuid() not null,
    name varchar not null unique,
    slug varchar not null,
    created_at timestamp without time zone default timezone('utc'::text, now()) not null,
    updated_at timestamp without time zone default timezone('utc'::text, now()) not null
);


create table if not exists orgs.members (
    org_id uuid,
    member_id uuid,
    role varchar default 'OWNER',

    primary key (org_id, member_id)
);