-- Your SQL goes here
create table bles (id serial primary key, title varchar(20) not null, idspace integer REFERENCES spaces(id) not null)