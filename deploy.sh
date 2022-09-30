#!/bin/bash

# This bash script starts the xmrbc-rs server

# Install rustlang
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env" && rustup update

# Clone and build nym
cd /home/dev/ && git clone https://github.com/nymtech/nym.git
 
# https://nymtech.net/docs/stable/run-nym-nodes/build-nym/
# Note: the default branch you clone from Github, `develop`, may be
# incompatible with both the mainnet and testnet. As such, make sure 
# to checkout the current release: 
cd nym && git checkout tags/nym-binaries-1.0.2 && cargo build --release
cp target/release/nym-client /usr/bin/
nym-client init --id server --port $1
nym-client run --id server &

# start xmrbc-rs
cd /home/dev/xmrbc-rs && cargo run --bin server -- --websocket ws://127.0.0.1:$1
