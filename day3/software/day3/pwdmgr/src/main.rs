mod models;
mod crypto;
mod storage;
use clap::{Parser, Subcommand};
use std::io::{self, Write};

use crate::crypto::{CryptoManager , CryptoError};
use crate::storage::{StorageError, Storage};
use crate::models::{PasswordEntry, PasswordStore};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a service
    Add {service: String, username: String},

    /// Get a service information
    Get {service: String},

    /// List all services
    List,

    /// Delete a service
    Delete{service: String},

    /// Generate a password (default size: 16)
    Generate {#[arg(default_value_t = 16)]length: usize}
}

fn main() {
    let cli = Cli::parse();

    print!("\nEnter the master password : ");
    io::stdout().flush().expect("Failed to flush");
    let master_password = rpassword::read_password().expect("Failed to read master password");
    let crypto = CryptoManager::new(&master_password).expect("Failed to create CryptoManager");
    let storage = Storage{crypto: crypto};
    
    match cli.command{
        Commands::Add { service, username } => {
            print!("\nEnter the password you want to add : ");
            io::stdout().flush().expect("Failed to flush");
            let password = rpassword::read_password().expect("Failed to read password");
            let mut store = storage.load().expect("Failed to load store");
            let new_entry = PasswordEntry{service: service, username: username, password: password};
            store.entries.push(new_entry);
            storage.save(&store).expect("Failed to save store");
            println!("\nService added successfully !\n")
        },
        Commands::Get { service } => {
            let store = storage.load().expect("Failed to load store");
            let target = store.entries.iter().find(|&s| s.service == service);
            if let Some(entry) = target {
                println!("\nService: {:?}", entry.service); 
                println!("Username: {:?}", entry.username); 
                println!("Password: {:?}\n", entry.password); 
            } else {
                println!("\nServices not registered !\n")
            }
        }
        Commands::List => {
            let store = storage.load().expect("Failed to load store");
            if store.entries.is_empty() {
                println!("\nYou have 0 services :(\n");
            } else {
                for entry in store.entries {
                    println!("\nService: {:?}", entry.service); 
                    println!("Username: {:?}", entry.username); 
                    println!("Password: {:?}\n", entry.password); 
                }
            }
        }
        Commands::Delete { service } => {
            let mut store = storage.load().expect("Failed to load store");
            store.entries.retain(|s| s.service != service);
            storage.save(&store).expect("Failed to save store");
            println!("\nService deleted successfully !")
        }
        Commands::Generate { length } => {
            let password = storage.generate_password(length);
            println!("Your generated password: {}", password);
        }
    };
}