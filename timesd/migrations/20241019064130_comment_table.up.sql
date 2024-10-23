-- Add up migration script here
create table times (
	id integer primary key autoincrement,
	title text not null,
	created_at datetime not null DEFAULT CURRENT_TIMESTAMP,
	updated_at datetime
);

create table posts(
  id integer primary key autoincrement,
  times_id integer not null,
  post text not null,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at datetime,
  foreign key(times_id) references times(id)
);
