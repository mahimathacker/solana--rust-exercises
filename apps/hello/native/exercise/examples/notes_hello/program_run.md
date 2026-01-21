cargo build-sbf
Test
cargo test -- --nocapture

cargo build-sbf --force-tools-install --tools-version v1.52

# Test with script 

Run local validator:

solana config set -ul
solana-test-validator


# Check program id

solana address -k ./target/deploy/hello-keypair.json
Deploy program

solana program deploy ./target/deploy/hello.so



# Deploy to Devnet
solana config set -ud
​
solana balance
# Airdrop if wallet balance is low
solana airdrop 1
​
cargo build-sbf
​
solana program deploy ./target/deploy/hello.so
​
PROGRAM_ID=your program ID
RPC=https://api.devnet.solana.com
KEYPAIR=path to key pair
​
cargo run --example demo $KEYPAIR $RPC $PROGRAM_ID


Close program to reclaim SOL

solana program close $PROGRAM_ID

