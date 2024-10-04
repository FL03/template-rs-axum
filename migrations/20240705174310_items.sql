-- This migration file is used to create the `items` table.
-- 

CREATE TABLE IF NOT EXISTS items
(
    id          uuid primary key default gen_random_uuid(),
    title       text not null,
    description text not null default '',
    created_at  timestamptz not null default now()
);