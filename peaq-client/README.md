# peaq-client

Rust PEAQ client to interact with PEAQ Substrate node & pallets.

## Usage

To get latest block use following command:

```shell
cargo test -- --nocapture get_latest_block
```


curl -sX POST -H "Content-Type: application/json" -d '{"id": 1, "jsonrpc": "2.0", "method": "peaqdid_readAttribute", "params": ["5CS3ZHVZRSKckfQ583aCszSsMiJ6F32kNUGgxTvzdTpdcrCh", "0x73746165782d696f642d646576696365", "0x05646eb05197d15821f157dd2a47d9fde6553f78096392bfed073bae8b242a25"]}' https://rpcpc1-qa.agung.peaq.network | jq .

sometimes params not array but object
