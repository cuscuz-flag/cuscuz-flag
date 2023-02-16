create table if not exists orgs.environments (
    id uuid default gen_random_uuid() not null,
    name text not null,
    org_id uuid not null,
    active boolean default true,
    created_at timestamp without time zone default timezone('utc'::text, now()) not null,
    updated_at timestamp without time zone default timezone('utc'::text, now()) not null,

    primary key (id),
    unique (org_id, name)
);

create table if not exists orgs.feature_flags (
    id uuid default gen_random_uuid() not null,
    name text not null,
    public_name text not null,
    description text,
    value boolean not null,
    env_id uuid not null,
    active boolean default true,
    created_at timestamp without time zone default timezone('utc'::text, now()) not null,
    updated_at timestamp without time zone default timezone('utc'::text, now()) not null,

    primary key (id),
    unique (env_id, name)
);
