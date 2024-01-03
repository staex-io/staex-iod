# Contracts

## Usage

### Build and deploy

```shell
make deploy name=did
```

### Call smart contract from CLI

Go to contract folder and execute:

```shell
cargo contract call --contract <address> --message flip --suri //Alice -x --skip-confirm
cargo contract call --contract <address> --message get --suri //Alice
```
