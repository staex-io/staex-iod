# StaexIoD

Staex Internet of Data.

The goal of this project is to create a Web3 IoT data infrastructure with a stable economy. In simple words we want to simplify Web3 onboarding process and bring more IoT device owners to share their useful data with some profit and other people to find and use such data transparently, securely and easily for their life or research.

[peaq-client](./peaq-client/) - contains Rust client to interact with PEAQ node \
[peaq-gen](./peaq-client/) - contains generated Rust source code from node metadata \
[provisioner](./provisioner/) - contains provisioner which can sync on-chain did and index pallets events \
[ui](./ui/) - user interface

## Usage

- Run [indexer](./provisioner/README.md)
- Run [provisioner](./provisioner/README.md)
- Run [UI](./ui/README.md)

## RBAC

![alt text](<images/peaq_rbac.png>)
