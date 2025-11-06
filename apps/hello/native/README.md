# Hello Solana (Native)

# Build

Generates `.so` file under `target/deploy`
```shell
cargo build-sbf
```

# Test with LiteSVM
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
cargo run --example demo $PROGRAM_ID
```
