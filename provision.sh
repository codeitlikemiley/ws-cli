#!/bin/sh

cargo clean
rm ws-cli.pkg
cargo zigbuild --release
cargo bundle --release
pkgbuild --root ./target/release/bundle/osx/ws-cli.app --install-location "/Applications/ws-cli.app" --identifier com.codeitlikemiley.ws-cli --version 0.1.0 --scripts ./scripts ws-cli.pkg
