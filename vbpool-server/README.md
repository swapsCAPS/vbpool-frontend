# vbpool-server

## TODO

- [-] Test PATCH
  - [x] Test null fields, currently not sending payload will result in `NOT_NULL` error
- [x] Test if deleting other user's forms is allowed
- [ ] Authentication
  - [ ] OA2P proxy
  - [ ] Keycloak

## Authentication Arch

```
     WWW     |        SERVER
             |
client <-> nginx -> oauth2-proxy -> vbpool-server
             |   |      |               |
             |    -> keycloak           |
             |          |               |
             |       postgres        postgres
```

### Run auth stack

Ensure dev certs have been created with [mkcert](https://github.com/FiloSottile/mkcert):

```
mkdir dev-certs
cd dev-certs
mkcert "*.local-vbpool.nl"
mkcert -install
```

Ensure following entries in `/etc/hosts`:

```
127.0.0.1	app.local-vbpool.nl
127.0.0.1	keycloak.local-vbpool.nl
```

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
