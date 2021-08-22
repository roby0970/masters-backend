-- Your SQL goes here
alter table coordinates alter column idspace set not null, add column idpoi integer REFERENCES pois(id) not null, add column blocked bool default false