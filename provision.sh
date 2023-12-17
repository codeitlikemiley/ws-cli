#!/bin/sh

cargo clean
rm workspacer.pkg
cargo build --release
cargo bundle --release
pkgbuild --root ./target/release/bundle/osx/Workspacer.app --install-location "/Applications/Workspacer.app" --identifier com.codeitlikemiley.workspacer --version 0.1.0 --scripts ./scripts workspacer.pkg
