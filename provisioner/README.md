# Provisioner

## Usage

```shell
cargo run -- --help
```

### Example config file

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
