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
    '{"data_type":"speed","location":"40.1949288120072,44.55177253802097","price_access":67.0362,"price_pin":998.1125,"staex_mcc_id":"ds4b7m9v3805e7gksjxm101k7pccqbg5bgdxzzt9ve6pgsqjwnfg","mqtt_topics":["vertical-speed","horizontal-speed"],"additional":{"microcontroller":"texas technologies"}}',
    1707473498
  ),
  (
    '5GR8gNHFkEh4Dm56iuZWve2N1GtUy3iwzSFUbTNcTpDELDPc',
    'v1',
    '{"data_type":"temperature","location":"24.6087342211612,54.68304923228258","price_access":5.5918,"price_pin":1,"staex_mcc_id":"g31te4yehrfrvdpwq7savca959f8yprwt333f4ez1rshx6ava230","mqtt_topics":["volcano","sea-air"],"additional":{"microcontroller":"stm32"}}',
    1707472498
  ),
  (
    '5DQ4DQH9Uxigrh2HSzmiPTUZji5KDitvrbYd3X3qeSuJ7MVU',
    'v1',
    '{"data_type":"emf","location":"52.56902231841478,13.355405168767573","price_access":33,"price_pin":5001.1,"staex_mcc_id":"c7h344s0j19xefxe3ejz7yhgz3pnr8cpw5tq8qnz05450g6vb33g","mqtt_topics":["measurements"],"additional":{"microcontroller":"raspberry pi"}}',
    1707474798
  ),
  (
    '5GsZYuBnJsxbmkETX7s4QHYCramdhm5oBgWyeXJnfvMroy6u',
    'v1',
    '{"data_type":"cctv-camera","location":"-41.29940519428393,174.79885750330672","price_access":99.14,"price_pin":901.23,"staex_mcc_id":"9pm270m01f5s6f6bkzmhnnjw5rz4bq80f5w2ceqnfdbq0st874ag","mqtt_topics":["inside","outside"],"additional":{"microcontroller":"raspberry pi zero w"}}',
    1707464498
  ),
  (
    '5HVhkvq3VZAbmCbYVSYeGYRiU3y7yxoh69RyQdxuQ1699Ebv',
    'v1',
    '{"data_type":"alcohol","location":"35.48916190115505,139.67532377317616","price_access":12.3,"price_pin":8891.1,"staex_mcc_id":"x0grdn1rf31k0m1yt40bnebb2j0mfyeacr2qmdfz5ywpa7bjxnp0","mqtt_topics":["vodka","wine"],"additional":{"microcontroller":"stm32","device_age_in_years":2}}',
    1707474982
  );
