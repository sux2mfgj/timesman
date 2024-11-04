-- Add up migration script here
alter table times add column flags integer not null default 0;
