# vbpool-server

## Testing
Since tests use a single DB to hold state we need to run them in series:
```rust
cargo test -- --test-threads=1
```
