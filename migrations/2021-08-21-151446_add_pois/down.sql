-- This file should undo anything in `up.sql`
alter table coordinates add column idpoi  integer references pois(id), drop column idspace