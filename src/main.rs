use clap::{Arg, Command};
use std::error::Error;
use std::fs;
use toml_edit::DocumentMut;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("ws-cli")
        .version("0.1.2")
        .author("Uriah <codeitlikemiley@gmail.com>")
        .about("Manage Rust workspace")
        .subcommand_required(true) // Require a subcommand
        .arg_required_else_help(true) // Show help if no subcommand is provided
        .subcommand(Command::new("init").about("Initializes a new workspace"))
        .subcommand(
            Command::new("add")
                .about("Adds a member to the workspace")
                .arg(
                    Arg::new("member")
                        .help("The member to add")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("rm")
                .about("Removes a member from the workspace")
                .arg(
                    Arg::new("member")
                        .help("The member to remove")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(Command::new("ls").about("Lists members of the workspace"))
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => init_workspace(),
        Some(("add", sub_m)) => {
            let member = sub_m.get_one::<String>("member").unwrap();
            add_member_to_workspace(member)
        }
        Some(("rm", sub_m)) => {
            let member = sub_m.get_one::<String>("member").unwrap();
            remove_member_from_workspace(member)
        }
        Some(("ls", _)) => list_members(),
        _ => Ok(()),
    }
}

fn list_members() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("Cargo.toml")?;
    let doc = content.parse::<DocumentMut>()?;

    let members = doc["workspace"]["members"]
        .as_array()
        .ok_or("Failed to read members as array")?;

    println!("Workspace members:");
    for member in members.iter() {
        println!("- {}", member.as_str().unwrap_or(""));
    }

    Ok(())
}

fn init_workspace() -> Result<(), Box<dyn Error>> {
    let content = r#"[workspace]
resolver = "2"
members = []
"#;
    fs::write("Cargo.toml", content)?;
    println!("Initialized new workspace");
    Ok(())
}

fn add_member_to_workspace(member: &str) -> Result<(), Box<dyn Error>> {
    let content = if fs::metadata("Cargo.toml").is_ok() {
        fs::read_to_string("Cargo.toml")?
    } else {
        println!("Cargo.toml not found, creating a new workspace");
        init_workspace()?;
        fs::read_to_string("Cargo.toml")?
    };

    let mut doc = content.parse::<DocumentMut>()?;

    let members = doc["workspace"]["members"]
        .as_array_mut()
        .ok_or("Failed to read members as array")?;

    if !members.iter().any(|m| m.as_str() == Some(member)) {
        members.push(member);
    }

    fs::write("Cargo.toml", doc.to_string())?;
    println!("Added member '{}' to workspace", member);
    Ok(())
}

fn remove_member_from_workspace(member: &str) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("Cargo.toml")?;
    let mut doc = content.parse::<DocumentMut>()?;

    let members = doc["workspace"]["members"]
        .as_array_mut()
        .ok_or("Failed to read members as array")?;

    // First, find the index outside of the mutable borrow scope.
    let index = members.iter().position(|m| m.as_str() == Some(member));

    if let Some(idx) = index {
        // Now we can safely remove the item.
        members.remove(idx);
    }

    fs::write("Cargo.toml", doc.to_string())?;
    println!("Removed member '{}' from workspace", member);
    Ok(())
}
