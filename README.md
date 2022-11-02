# Nym Monero Broadcaster

[![Rust](https://github.com/hyahatiph-labs/xmrbc-rs/actions/workflows/rust.yml/badge.svg?branch=dev)](https://github.com/hyahatiph-labs/xmrbc-rs/actions/workflows/rust.yml)

This repository implements an anonymous Monero transaction broadcasting tool on top of
[Nym](https://github.com/nymtech/nym), a mixnet still under heavy development. So while the technology looks promising
any anonymity claims are to be taken with a grain of salt. This project is provided as-is, it might work as expected or
not, please don't rely on it without vetting it yourself. 

There are two parts:
* **Client:** connects to a nym native-client and sends transaction to a specified server (aka service provider)
* **Server:** listens for incoming nym packets from its nym native-client. If they are valid client requests containing a valid transaction, 
it is broadcasted to its respective network using [xmrchain.net](https://xmrchain.net/) and the [Onion Monero Block Explorer](https://github.com/moneroexamples/onion-monero-blockchain-explorer), or enter your own block explorer url with the `--network` flag.

## Usage
### Monero tx blob

Sample RPC call to get `tx_blob`

```bash
curl http://127.0.0.1:18083/json_rpc -d '{"jsonrpc":"2.0","id":"0","method":"transfer","params":{"destinations":[{"amount":1000000000,"address":"4abc..."}],"account_index":0,"subaddr_indices":[0],"priority":3,"ring_size":16, "do_not_relay": true, "get_tx_hex": true}}' -H 'Content-Type: application/json'
```

### Nym Native Client

To use either one you have to initialize and run a [Nym client](https://nymtech.net/docs/stable/developers/develop-with-nym/websocket-client):

```bash
nym-client init --id client
nym-client run --id client
```

If you want to run both client and server on one machine it's advisable to run two nym clients on different ports:

```bash
nym-client init --id client # default port = 1977
nym-client init --id server --port 1978

nym-client run --id client
nym-client run --id server
``` 

### XMR-BC Client

```bash
xmrbc 0.1.0

USAGE:
    client [OPTIONS] <transaction>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --network <network>
            enter block explorer url [default:
            https://xmrchain.net/checkandpush]
    -s, --service-provider <service-provider>
             [default: 7DmUkap6s3CK2kJnYsCe7kZu2QzjnB1MA3tiRKD6gcPx.3e2EuyuMFPLVQvAZ9nM75epUegZjvh2wz2HNnjVJBLLR@678qVUJ21uwxZBhp3r56z7GRf6gMh3NYDHruTegPtgMf]
    -w, --websocket <websocket>                   [default: ws://127.0.0.1:1977]

ARGS:
    <transaction>   
```

If you cloned this repo, have [Rust installed](https://rustup.rs/) and initialized your nym client as shown above you
can run the following to transmit Monero tx transaction `<transaction>` through a service provider at `<address>`:

```bash
cargo run --bin client -- -s <address> <transaction>
```

There is a default service provider at `7DmUkap6s3CK2kJnYsCe7kZu2QzjnB1MA3tiRKD6gcPx.3e2EuyuMFPLVQvAZ9nM75epUegZjvh2wz2HNnjVJBLLR@678qVUJ21uwxZBhp3r56z7GRf6gMh3NYDHruTegPtgMf`

which I run on a best-effort basis and which is chosen if the `-s` flag isn't provided. Please don't rely on it for anything critical.

If you want to transmit it to another `<network>` just specify the network
flag with the block explorer url:

```bash
cargo run --bin client -- --network <network> -s <address> <transaction>
```

### XMR-BC Server

```bash
xmrbc 0.1.0

USAGE:
    server [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -w, --websocket <websocket>     [default: ws://127.0.0.1:1977]
```

If you cloned this repo, have [Rust installed](https://rustup.rs/) and initialized your nym client as shown above you
can run the following to start the server:

```bash
cargo run --bin server -- --websocket ws://127.0.0.1:1978
```

It will output a log message telling you its nym address:

```bash
Feb 13 15:07:20.291  INFO server: Listening on 7DmUkap6s3CK2kJnYsCe7kZu2QzjnB1MA3tiRKD6gcPx.3e2EuyuMFPLVQvAZ9nM75epUegZjvh2wz2HNnjVJBLLR@678qVUJ21uwxZBhp3r56z7GRf6gMh3NYDHruTegPtgMf
```

This address has to be given as an argument to the client when sending transaction.

### XMR-BC rpc

```bash
xmrbc 0.1.0

USAGE:
    rpc [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --network <network>
            enter block explorer url [default:
            https://stagenet.xmrchain.net/checkandpush]
    -s, --service-provider <service-provider>
             [default: D5ehV2EdXb8LcPZM5kNL1C1D61pcJqzKRKNsg2hrB6ig.3iArACGGkrd5unbmqHBJkAWNqNRd29EA79aFWJZUxGwR@ERFGo6CbzLe51EZUgGJoYg2zsYnVXVVfLyB5CZ8P9vsU]
    -w, --websocket <websocket>                   [default: ws://127.0.0.1:1977]
```

If you cloned this repo, have [Rust installed](https://rustup.rs/) and initialized your nym client as shown above you
can run the following to start the server:

```bash
cargo run --bin rpc
```

```bash
http://127.0.0.1:8000/relay?tx=<TX_BLOB>
```


## Debugging
If something isn't working as expected you can use the `RUST_LOG` environment variable to enable more verbose logging
(e.g. `RUST_LOG=debug`).

Sample success [log](./success-log.md)

## Docker

```bash
podman build --network host -t xmrbc-rs:v0.1.0 .
```

```bash
podman run --rm -P -p 127.0.0.1:<SERVER_PORT>:<SERVER_PORT> --name xmrbc-rs \
xmrbc-rs:v0.1.0 /bin/bash -c "sh /home/dev/deploy.sh <SERVER_PORT>"
```

## Diesel / Postgresql

* WIP
* see the [diesel docs](https://crates.io/crates/diesel_cli)
* recommend using postgres [docker](https://hub.docker.com/_/postgres)

 
## Shoutouts
* This was inspired by @sgeisler 's work on [btcbc-rs](https://github.com/sgeisler/btcbc-rs)
* Thanks to @t-900-a for suggesting
