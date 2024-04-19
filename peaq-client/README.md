# peaq-client

Rust PEAQ client to interact with PEAQ Substrate node & pallets.

## Usage

To get latest block use following command:

```shell
cargo test -- --nocapture get_latest_block
```

## RPC requests examples

Sometimes as params we need to use an array but sometimes an object.

To read DID attribute:

```shell
curl -sX POST -H "Content-Type: application/json" -d '{"id": 1, "jsonrpc": "2.0", "method": "peaqdid_readAttribute", "params": ["5CS3ZHVZRSKckfQ583aCszSsMiJ6F32kNUGgxTvzdTpdcrCh", "0x73746165782d696f642d646576696365", "0x05646eb05197d15821f157dd2a47d9fde6553f78096392bfed073bae8b242a25"]}' https://rpcpc1-qa.agung.peaq.network | jq .
```

To fetch user permissions:

```shell
curl -s -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "peaqrbac_fetchUserPermissions", "params": ["5CS3ZHVZRSKckfQ583aCszSsMiJ6F32kNUGgxTvzdTpdcrCh", [122, 190, 82, 250, 244, 222, 128, 103, 71, 215, 50, 122, 3, 178, 251, 167, 35, 47, 138, 5, 239, 66, 202, 72, 78, 51, 242, 157, 60, 181, 104, 107], "0x05646eb05197d15821f157dd2a47d9fde6553f78096392bfed073bae8b242a25"]}' https://rpcpc1-qa.agung.peaq.network | jq .
```

To fetch all owner permissions:

```shell
curl -s -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "peaqrbac_fetchPermissions", "params": ["5CS3ZHVZRSKckfQ583aCszSsMiJ6F32kNUGgxTvzdTpdcrCh", "0x05646eb05197d15821f157dd2a47d9fde6553f78096392bfed073bae8b242a25"]}' https://rpcpc1-qa.agung.peaq.network | jq .
```
