-- Add up migration script here
create table comments(
  id integer primary key autoincrement,
  comment text not null,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
