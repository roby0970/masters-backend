-- Your SQL goes here
CREATE TABLE pois (
    id serial primary key,
    title varchar(100) not null,
    spaceid integer references spaces(id)
)

