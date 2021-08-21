-- This file should undo anything in `up.sql`
alter table spaces alter column longitude drop not null, alter column latitude drop not null 