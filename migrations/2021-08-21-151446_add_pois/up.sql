-- Your SQL goes here
alter table coordinates drop column idpoi, add column idspace integer references spaces(id)
