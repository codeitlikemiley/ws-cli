# A CLI tool (ws-cli) to Manage GRPC Services Workspace

[![Release](https://github.com/codeitlikemiley/ws-cli/actions/workflows/rust_build.yml/badge.svg)](https://github.com/codeitlikemiley/ws-cli/actions/workflows/rust_build.yml)

Note: This is an additional tooling to help you be productive building GRPC Server with Rust

Mainly used to Initialize a new Workpspace for [Server Template](https://github.com/codeitlikemiley/server_template) and add Service with [Service Template](https://github.com/codeitlikemiley/service_template)

## Installation

1. You can Download and Install [workspacer cli](https://github.com/codeitlikemiley/ws-cli/releases) on Releases Page

Note: on MacOS you might need to go to System Preferences > Security & Privacy > General and click Open Anyway to install it

Note: on Windows you might need to Add the command to ENV PATH


2. Install via Cargo , by default it would install both `ws` and `ws-cli` command, the command below would only install `ws` command

```sh
cargo install ws-cli --bin ws
```


## Developer Workflow

1. Initialize a new workspace

```sh
mkdir workspace
cd workspace
ws init
```

2. Create Server Template

```sh
cd workspace
cargo generate --git codeitlikemiley/server_template --name server
```

3. Generate Services

```sh
mkdir services
ws add auth
cd services
cargo generate --git codeitlikemiley/services_template --name auth
```

> CLI Example Usage

```sh
ws

Manage workspace for GRPC services

Usage: ws <COMMAND>

Commands:
  init    Initializes a new workspace
  add     Adds a member to the workspace
  remove  Removes a member from the workspace
  ls      Lists members of the workspace
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
