# Hello Solana (Native)

Create an empty folder and complete all tasks below
- Build
- Test locally with LiteSVM
- Deploy locally to `solana-test-validator` and test with Rust script
- Deploy to Devnet and run the Rust script again

# Build

Generates `.so` file under `target/deploy`
```shell
curl -L -o ~/Downloads/platform-tools-osx-aarch64.tar.bz2 https://github.com/anza-xyz/platform-tools/releases/download/v1.52/platform-tools-osx-aarch64.tar.bz2

rm -rf ~/.cache/solana/v1.52
mkdir -p ~/.cache/solana/v1.52/platform-tools

cd ~/.cache/solana/v1.52/platform-tools
tar -xjf ~/Downloads/platform-tools-osx-aarch64.tar.bz2
ls ~/.cache/solana/v1.52/platform-tools/

Step 1: Backup the old platform-tools
bashmv /Users/mahimathacker/.local/share/solana/install/releases/stable-90098d261e2be2f898769d9ee35141597f1a2234/solana-release/bin/platform-tools-sdk/sbf/dependencies/platform-tools /Users/mahimathacker/.local/share/solana/install/releases/stable-90098d261e2be2f898769d9ee35141597f1a2234/solana-release/bin/platform-tools-sdk/sbf/dependencies/platform-tools-v1.51-backup
Step 2: Link to v1.52
bashln -s ~/.cache/solana/v1.52/platform-tools /Users/mahimathacker/.local/share/solana/install/releases/stable-90098d261e2be2f898769d9ee35141597f1a2234/solana-release/bin/platform-tools-sdk/sbf/dependencies/platform-tools
Step 3: Verify
bashcargo build-sbf --version
Should now show v1.52!
Step 4: Build your program
bashcd /Users/mahimathacker/solana-course/apps/hello/native/exercise
cargo clean


cargo build-sbf --tools-version v1.52```

# Test
```shell
cargo test -- --nocapture
```

# Test with script

Run local validator
```shell
solana config set -ul
solana-test-validator
```

Check program id
```shell
solana address -k ./target/deploy/hello-keypair.json
```

Deploy program
```shell
solana program deploy ./target/deploy/hello.so
```

Execute demo script
```shell
PROGRAM_ID=your program ID
RPC=http://localhost:8899
KEYPAIR=path to key pair

cargo run --example demo $KEYPAIR $RPC $PROGRAM_ID
```

# Deploy to Devnet

```shell
solana config set -ud

solana balance
# Airdrop if wallet balance is low
solana airdrop 1

cargo build-sbf

solana program deploy ./target/deploy/hello.so

PROGRAM_ID=your program ID
RPC=https://api.devnet.solana.com
KEYPAIR=path to key pair

cargo run --example demo $KEYPAIR $RPC $PROGRAM_ID
```

Check transaction signature at [Solana explorer](https://explorer.solana.com/?cluster=devnet)

Close program to reclaim SOL
```shell
solana program close $PROGRAM_ID
```
