-- Add up migration script here
create table posts(
  id integer primary key autoincrement,
  tid integer not null,
  post text not null,
  created_at datetime not null DEFAULT CURRENT_TIMESTAMP,
  updated_at datetime,
  foreign key(tid) references times(id)
);
