# A CLI tool to Manage GRPC Services Workspace

Note: This is an additional tooling to help you be prodctive building GRPC Server with Rust

Mainly used to Initialize a new Workpspace for [Server Template](https://github.com/codeitlikemiley/server_template) and add Service with [Service Template](https://github.com/codeitlikemiley/service_template)

## Installation

- If you are on Mac OS , you can Download and Install [workspacer.pkg](./workspacer.pkg)


- If you on other Platform you can build it from source
```sh
git clone htps://github.com/codeitlikemiley/workspacer.git
cd workspacer
cargo build --release
```

- On any Linux or Unix based system you can install it with the following commands:

```sh
mv ./target/release/workspacer /usr/local/bin/workspacer
chmod +x /usr/local/bin/workspacer
```

- On windows , open terminal

```powershell
# Replace 'YourUsername' with your actual username
Move-Item .\target\release\workspacer.exe C:\Users\YourUsername\bin\workspacer.exe

# Again, replace 'YourUsername' with your actual username
$env:Path += ";C:\Users\YourUsername\bin"
```

## Developer Workflow

1. Initialize a new workspace

```sh
mkdir workspace
cd workspace
workspacer init
```

2. Create Server Template

```sh
cd workspace
cargo generate --git codeitlikemiley/server_template --name server
```

3. Generate Services

```sh
mkdir services
workspacer add auth
cd services
cargo generate --git codeitlikemiley/services_template --name auth
```

> CLI Example Usage

```sh
workspacer

Manage workspace for GRPC services

Usage: workspacer <COMMAND>

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
