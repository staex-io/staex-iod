create table devices (
  address text primary key,
  version text not null,
  data json not null,
  updated_at integer not null
);

insert into
  devices (address, version, data, updated_at)
values
  (
    '5D1nDaTRBvyJTW4t3T8hmehftxr93SiS227hDxPDYvGnE4ZZ',
    'v1',
    '{"data_type":"cctv-camera","location":"40.1949288120072,44.55177253802097","price_access":42.03995,"pin_access":445.12222,"additional":{"microcontroller":"stm32","device_age_in_years":2}}',
    1707473498
  ),
  (
    '5GR8gNHFkEh4Dm56iuZWve2N1GtUy3iwzSFUbTNcTpDELDPc',
    'v1',
    '{"data_type":"cctv-camera","location":"24.6087342211612,54.68304923228258","price_access":42.03995,"pin_access":445.12222,"additional":{"microcontroller":"stm32","device_age_in_years":2}}',
    1707472498
  ),
  (
    '5DQ4DQH9Uxigrh2HSzmiPTUZji5KDitvrbYd3X3qeSuJ7MVU',
    'v1',
    '{"data_type":"cctv-camera","location":"52.56902231841478,13.355405168767573","price_access":42.03995,"pin_access":445.12222,"additional":{"microcontroller":"stm32","device_age_in_years":2}}',
    1707474798
  ),
  (
    '5GsZYuBnJsxbmkETX7s4QHYCramdhm5oBgWyeXJnfvMroy6u',
    'v1',
    '{"data_type":"cctv-camera","location":"-41.29940519428393,174.79885750330672","price_access":42.03995,"pin_access":445.12222,"additional":{"microcontroller":"stm32","device_age_in_years":2}}',
    1707464498
  ),
  (
    '5HVhkvq3VZAbmCbYVSYeGYRiU3y7yxoh69RyQdxuQ1699Ebv',
    'v1',
    '{"data_type":"cctv-camera","location":"35.48916190115505,139.67532377317616","price_access":42.03995,"pin_access":445.12222,"additional":{"microcontroller":"stm32","device_age_in_years":2}}',
    1707474982
  );
