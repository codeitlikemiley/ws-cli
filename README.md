# A CLI tool to Manage GRPC Services Workspace

Note: This is an additional tooling to help you be prodctive building GRPC Server with Rust

Mainly used to Initialize a new Workpspace for [Server Template](https://github.com/codeitlikemiley/server_template) and add Service with [Service Template](https://github.com/codeitlikemiley/service_template)

## Installation

1. You can Download and Install [workspacer](https://github.com/codeitlikemiley/workspacer/releases) on Releases Page

Note: on MacOS you might need to go to System Preferences > Security & Privacy > General and click Open Anyway to install it

Note: on Windows you might need to Add the command to ENV PATH


2. Build it from source


Clone

```sh
git clone htps://github.com/codeitlikemiley/workspacer.git
cd workspacer
```

**For MacOS**
```sh
./provision.sh

```

**For Linux**

```sh
cargo build --release
mv ./target/release/workspacer /usr/local/bin/workspacer
chmod +x /usr/local/bin/workspacer
```

**For Windows**

```powershell
cargo build --release

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
