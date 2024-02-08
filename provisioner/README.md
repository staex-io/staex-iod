# Provisioner

## Usage

```shell
cargo run -- --help
```

### Example config file

Config file should be location in provisioner root folder for default file path value.

```shell
cargo run -- config
```

```toml
log_level = "DEBUG"
rpc_url = "ws://127.0.0.1:9944"

[signer]
typ = "SecretUri"
val = "//Alice"

[faucet]
amount = 999993264201726756

[faucet.signer]
typ = "SecretUri"
val = "//Alice"

[device]
sync = true

[device.attributes]
data_type = "cctv-camera"
location = "40.1949288120072,44.55177253802097"
price_access = "42.03995"
pin_access = "445.12222"

[indexer]
from_block = 1717233
dsn = "sqlite:staex-iod.sqlite"
host = "127.0.0.1"
port = 4698
```

### ink! smart contracts research

In [main.rs.old.txt](./src/main.rs.old.txt) research about interaction with ink! smart contracts located.

### Get devices by HTTP API

```shell
curl -s X GET 'http://127.0.0.1:4698/devices?limit=10&offset=1&data_type=cctv-camera&price_access=42.03995' | jq
```

```json
[
  {
    "address": "5CwQRPkqmUg5arWuJtw2qoGRL4oRDjguzmrcrSSsv35Cuv3s",
    "version": "v1",
    "device": {
      "data_type": "cctv-camera",
      "location": "40.1949288120072,44.55177253802097",
      "pin_access": "445.12222",
      "price_access": "42.03995"
    },
    "updated_at": 1707386161
  }
]
```
