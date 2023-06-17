# vbpool-server

## TODO
- [-] Test PATCH
  - [x] Test null fields, currently not sending payload will result in `NOT_NULL` error
- [x] Test if deleting other user's forms is allowed
- [ ] Authentication
  - [ ] OA2P proxy
  - [ ] Ory hydra

## Authentication Arch
```
nginx -> oauth2-proxy -> vbpool-server
              |               |
           keycloak           |
              |               |
           postgres ----------
```

### Initialize tables (run once)
```
docker compose --profile ory-hydra-init up
```
Postgres tables for Ory are now initialized

### Create oauth2 client app (run once)
```
docker compose --profile ory-hydra-create-client up
```
Copy paste resulting client credentials into .env file

### Run auth stack
```
docker compose up
```
Auth stack is running

## Testing
Since tests use a single DB to hold state we need to run them in series using:
```bash
cargo test -- --test-threads=1
```

You can also use
```bash
make test
```
