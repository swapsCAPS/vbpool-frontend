# vbpool-server

## TODO
- [-] Test PATCH
  - [x] Test null fields, currently not sending payload will result in `NOT_NULL` error
- [x] Test if deleting other user's forms is allowed
- OA2P proxy

## OA2P Proxy
```
nginx -> OA2P-P -> vbpool-server
```

## Testing
Since tests use a single DB to hold state we need to run them in series using:
```bash
cargo test -- --test-threads=1
```

You can also use
```bash
make test
```
