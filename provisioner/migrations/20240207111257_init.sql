create table dids (
  address text primary key,
  data json not null,
  updated_at integer not null
);
