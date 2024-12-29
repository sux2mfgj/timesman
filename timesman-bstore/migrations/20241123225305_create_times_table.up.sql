-- Add up migration script here
create table times (
  id integer primary key autoincrement,
  title text not null,
  created_at datetime not null DEFAULT CURRENT_TIMESTAMP,
  updated_at datetime,
  deleted integer not null DEFAULT 0
);
