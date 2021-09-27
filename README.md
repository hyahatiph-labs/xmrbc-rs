# Nym Monero Broadcaster
This repository implements an anonymous Monero metadata broadcasting tool on top of
[Nym](https://github.com/nymtech/nym), a mixnet still under heavy development. So while the technology looks promising
any anonymity claims are to be taken with a grain of salt. This project is provided as-is, it might work as expected or
not, please don't rely on it without vetting it yourself. 

There are two parts:
* **Client:** connects to a nym native-client and sends metadata to a specified server (aka service provider)
* **Server:** listens for incoming nym packets from its nym native-client. If they are valid client requests containing
metadata, it is broadcasted to its respective network using [Hyahatiph Labs](https://hiahatf.org/xmr-relay/) xmr relay API.

## Usage
### Nym Native Client
To use either one you have to initialize and run a [Nym client](https://nymtech.net/docs/build-peapps/native-client/):

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
```
xmrbc 0.1.0

USAGE:
    client [OPTIONS] <metadata>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --network <network>                      one of 'monero' or 'stagenet' [default: monero]
    -s, --service-provider <service-provider>
             [default: TBD ]
    -w, --websocket <websocket>                   [default: ws://127.0.0.1:1977]

ARGS:
    <metadata>    
```

If you cloned this repo, have [Rust installed](https://rustup.rs/) and initialized your nym client as shown above you
can run the following to transmit Monero tx metadata `<metadata>` through a service provider at `<address>`:

```
cargo run --bin client -- -s <address> <metadata>
```

There is a default service provider at `5NkuNyAUkeMZQRxbb77FxXHCTUti1tgFdRSYkXxvycP4.G9J5L4CkJS7qoirQvfxVDGvRSjn3ANjHhMopK5i3CH4E@6LdVTJhRfJKsrUtnjFqE3TpEbCYs3VZoxmaoNFqRWn4x
`
which I run on a best-effort basis and which is chosen if the `-s` flag isn't provided. Please don't rely on it for anything critical.

If you want to transmit it to another `<network>` (supported networks: monero, stagenet), just specify the network
flag:

```
cargo run --bin client -- --network <network> -s <address> <metadata>
```

### XMR-BC Server
```
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

```
cargo run --bin server -- --websocket ws://127.0.0.1:1978
```

It will output a log message telling you its nym address:

```
Feb 13 15:07:20.291  INFO server: Listening on 5NkuNyAUkeMZQRxbb77FxXHCTUti1tgFdRSYkXxvycP4.G9J5L4CkJS7qoirQvfxVDGvRSjn3ANjHhMopK5i3CH4E@6LdVTJhRfJKsrUtnjFqE3TpEbCYs3VZoxmaoNFqRWn4x
```

This address has to be given as an argument to the client when sending metadata.

## Debugging
If something isn't working as expected you can use the `RUST_LOG` environment variable to enable more verbose logging
(e.g. `RUST_LOG=debug`).

## Shoutouts
* This was inspired by @sgeisler 's work on [btcbc-rs](https://github.com/sgeisler/btcbc-rs)
* Thanks to @t-900-a for suggesting
