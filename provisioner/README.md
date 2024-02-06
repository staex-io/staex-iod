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
secret_uri = "//Alice"
amount = 100000

[did]
sync = true
explorer = true
contract_address = "5H4UGYpLFL2aobsv71CsiFwfcXe9yoSMGtrc6VENGzGRyQZa"
metadata_path = "did.metadata.json"

[did.attributes]
data_type = ""
location = ""
price_access = ""
pin_access = ""
```

### ink! smart contracts research

In [main.rs.old.txt](./src/main.rs.old.txt) research about interaction with ink! smart contracts located.
