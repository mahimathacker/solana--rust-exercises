# Course setup

## [Installation](https://solana.com/docs/intro/installation)

```shell
Rust: rustc 1.87.0
Solana CLI: solana-cli 3.0.10
Anchor CLI: anchor-cli 0.31.1
```

## [Solana CLI](https://solana.com/docs/intro/installation/solana-cli-basics)


```shell
# Get current configuration
solana config get

# Change CLI cluster (validators)
solana config set --url mainnet-beta
solana config set --url devnet
solana config set --url localhost
solana config set --url testnet
```

## Setup wallet

```
# Create a wallet
solana-keygen new

# Check wallet address
solana address

# Airdop SOL (Devnet)
solana config set -ud
solana airdrop 2

# Check balance
solana balance
```

[Solana web faucet](https://faucet.solana.com/)

## Run local validator
```shell
solana config set -ul
solana-test-validator
```
## Exercises

- Native and Anchor exercises
- All exercises and solutions are under [`apps`](../apps)
- Typical folder organization
    ```shell
    apps
    ├── hello
    │   ├── anchor
    │   │   ├── exercise
    │   │   ├── README.md
    │   │   └── solution
    │   └── native
    │       ├── exercise
    │       ├── README.md
    │       └── solution
    ```
- `README.md` are the exercises

