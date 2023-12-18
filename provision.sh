#!/bin/sh

cargo clean
rm ws.pkg
cargo zigbuild --release
cargo bundle --release
pkgbuild --root ./target/release/bundle/osx/ws.app --install-location "/Applications/ws.app" --identifier com.codeitlikemiley.ws --version 0.1.0 --scripts ./scripts ws.pkg
