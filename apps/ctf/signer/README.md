# Missing signer

The oracle program is designed so that only the owner of the oracle account can update the price.

But there is a bug in this program. Find the bug and update the price without the owner's authorization.

# Task - Write your exploit

Write your exploit inside [`test`]

# Build

```shell
cargo build-sbf
```

# Test with LiteSVM
```shell
cargo test -- --nocapture
```

