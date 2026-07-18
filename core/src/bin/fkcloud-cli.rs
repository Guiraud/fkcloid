use std::io::{self, Write};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use fkcloud_core::{RmfcClient, RmfcSession, Entry};

#[derive(Serialize, Deserialize)]
struct ConfigData {
    host: String,
    email: String,
    token: String,
}

fn config_path() -> Option<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()?;
    Some(PathBuf::from(home).join(".fkcloud-session.json"))
}

fn prompt_password() -> String {
    print!("Password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    password.trim().to_string()
}

fn load_client() -> Result<RmfcClient, String> {
    // 1. Check environment variables first
    let env_host = std::env::var("FKCLOUD_HOST").ok();
    let env_email = std::env::var("FKCLOUD_EMAIL").ok();
    let env_password = std::env::var("FKCLOUD_PASSWORD").ok();

    if let (Some(host), Some(email), Some(password)) = (env_host, env_email, env_password) {
        let session = RmfcSession::new(&host, &email, &password, true)
            .map_err(|e| format!("Invalid session configuration from environment: {}", e))?;
        return Ok(RmfcClient::new(session));
    }

    // 2. Fallback to config file
    let path = config_path()
        .ok_or_else(|| "Could not determine home directory".to_string())?;
    if !path.exists() {
        return Err("No active session. Please login first using: fkcloud-cli login <url> <email>".to_string());
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;
    let config: ConfigData = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config file: {}", e))?;

    let mut session = RmfcSession::new(&config.host, &config.email, "", true)
        .map_err(|e| format!("Invalid session configuration: {}", e))?;
    session.set_token(&config.token);

    Ok(RmfcClient::new(session))
}

fn print_entry(entry: &Entry, depth: usize) {
    let indent = "  ".repeat(depth);
    let prefix = if entry.is_folder { "📁" } else { "📄" };
    let info = if entry.is_folder {
        String::new()
    } else {
        let size_str = entry.size.map(|s| format!(" ({} bytes)", s)).unwrap_or_default();
        let doc_type_str = entry.doc_type.as_ref().map(|t| format!(" [{}]", t)).unwrap_or_default();
        format!("{}{}", doc_type_str, size_str)
    };
    println!("{}{}{} {} - ID: {}", indent, prefix, entry.name, info, entry.id);
    for child in &entry.children {
        print_entry(child, depth + 1);
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  fkcloud-cli login <url> <email>");
    println!("  fkcloud-cli ls");
    println!("  fkcloud-cli put <file_path> [--folder <parent_id>]");
    println!("  fkcloud-cli sync");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let command = &args[1];
    match command.as_str() {
        "login" => {
            if args.len() < 4 {
                println!("Error: login command requires <url> and <email>");
                print_usage();
                std::process::exit(1);
            }
            let host = &args[2];
            let email = &args[3];
            
            // Check if password in environment, else prompt
            let password = std::env::var("FKCLOUD_PASSWORD").unwrap_or_else(|_| prompt_password());

            let session = RmfcSession::new(host, email, &password, true)?;
            let client = RmfcClient::new(session);
            
            println!("Logging in to {}...", host);
            match client.login() {
                Ok(token) => {
                    let path = config_path()
                        .ok_or_else(|| "Could not determine home directory")?;
                    let config_data = ConfigData {
                        host: host.clone(),
                        email: email.clone(),
                        token,
                    };
                    let json = serde_json::to_string_pretty(&config_data)?;
                    std::fs::write(&path, json)?;
                    println!("Login successful! Session saved to {:?}", path);
                }
                Err(e) => {
                    eprintln!("Login failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "ls" => {
            let client = match load_client() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };

            match client.get_documents() {
                Ok(tree) => {
                    println!("Entries:");
                    for entry in &tree.entries {
                        print_entry(entry, 0);
                    }
                    if !tree.trash.is_empty() {
                        println!("\nTrash:");
                        for entry in &tree.trash {
                            print_entry(entry, 0);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error fetching documents: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "put" => {
            if args.len() < 3 {
                println!("Error: put command requires <file_path>");
                print_usage();
                std::process::exit(1);
            }
            let file_path = &args[2];
            let mut folder_id = "root";

            if let Some(pos) = args.iter().position(|x| x == "--folder") {
                if pos + 1 < args.len() {
                    folder_id = &args[pos + 1];
                } else {
                    eprintln!("Error: Missing folder ID after --folder");
                    std::process::exit(1);
                }
            }

            let client = match load_client() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };

            println!("Uploading {} to folder '{}'...", file_path, folder_id);
            match client.upload_document(folder_id, file_path) {
                Ok(_) => {
                    println!("Upload successful!");
                }
                Err(e) => {
                    eprintln!("Upload failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "sync" => {
            let client = match load_client() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };

            println!("Requesting sync...");
            match client.sync() {
                Ok(_) => {
                    println!("Sync request completed successfully!");
                }
                Err(e) => {
                    eprintln!("Sync failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            println!("Unknown command: {}", command);
            print_usage();
            std::process::exit(1);
        }
    }

    Ok(())
}
