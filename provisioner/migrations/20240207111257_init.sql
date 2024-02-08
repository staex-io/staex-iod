create table devices (
  address text primary key,
  version text not null,
  data json not null,
  updated_at integer not null
);
