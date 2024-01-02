# Staex IoD

Staex Internet of Data.

The goal of this project is to create a Web3 IoT data infrastructure with a stable economy. In simple words we want to simplify Web3 onboarding process and bring more IoT device owners to share their useful data with some profit and other people to find and use such data transparently, securely and easily for their life or research.

## Usage

```shell
# From project root folder.
rm -rf data-substrate ; make run_substrate
cd contracts
make deploy name=did
# Get contract address from output.
cd did
cargo contract call --contract <address> --message flip --suri //Alice -x --skip-confirm
cd ../../provisioner
cargo run
cd ..
```

