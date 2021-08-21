-- Your SQL goes here
create table coordinates (
    id serial primary key,
    x integer not null,
    y integer not null,
    idpoi integer REFERENCES pois(id)
)