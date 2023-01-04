# vbpool-server

## Testing
Since tests use a single DB to hold state we need to run them in series using:
```bash
cargo test -- --test-threads=1
```

You can also use
```bash
make test
```
