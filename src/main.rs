mod memory;
mod scanner;
mod process;

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::io::{self, Write};

/// A simple cheat engine clone written in Rust
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Attach to a process
    Attach {
        /// Name of the process to attach to
        process_name: String,
    },
    /// Scan for a specific value in memory
    Scan {
        /// The value to search for
        value: String,
        /// Use fuzzy matching with specified tolerance
        #[arg(short, long)]
        fuzzy: Option<f32>,
    },
    /// Modify a value in memory
    Modify {
        /// The address to modify
        address: String,
        /// The new value to write
        new_value: String,
    },
    /// List running processes
    List,
    /// Start interactive mode
    Interactive,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    match &args.command {
        Some(Commands::Attach { process_name }) => {
            println!("Attaching to process: {}", process_name);
            let process = process::attach(process_name)?;
            println!("Successfully attached to process: {}", process);
            println!("Note: To continue working with this process, use interactive mode:");
            println!("  ./cheated_cheat_engine interactive");
        }
        Some(Commands::Scan { value: _, fuzzy: _ }) => {
            println!("Error: Scan command requires an attached process.");
            println!("Please use interactive mode to attach and scan in the same session:");
            println!("  ./cheated_cheat_engine interactive");
        }
        Some(Commands::Modify { address: _, new_value: _ }) => {
            println!("Error: Modify command requires an attached process.");
            println!("Please use interactive mode to attach and modify in the same session:");
            println!("  ./cheated_cheat_engine interactive");
        }
        Some(Commands::List) => {
            println!("Listing running processes...");
            let processes = process::list_processes()?;
            for process in processes {
                println!("  {}", process);
            }
        }
        Some(Commands::Interactive) => {
            interactive_mode()?;
        }
        None => {
            println!("No command specified. Use --help for usage information.");
            println!("For interactive mode, use: ./cheated_cheat_engine interactive");
        }
    }
    
    Ok(())
}

fn interactive_mode() -> Result<()> {
    println!("Cheated Cheat Engine - Interactive Mode");
    println!("Type 'help' for available commands or 'exit' to quit.");
    
    let mut attached_process: Option<process::Process> = None;
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0].to_lowercase();
        
        match command.as_str() {
            "help" | "h" => {
                println!("Available commands:");
                println!("  attach <process_name>  - Attach to a process");
                println!("  list                  - List running processes");
                println!("  scan <value>          - Scan for a value in attached process");
                println!("  scan <value> --fuzzy <tolerance> - Fuzzy scan for a value");
                println!("  modify <address> <new_value> - Modify a value at address");
                println!("  help                  - Show this help");
                println!("  exit                  - Exit interactive mode");
            }
            "exit" | "quit" | "q" => {
                println!("Exiting interactive mode.");
                break;
            }
            "list" | "l" => {
                println!("Listing running processes...");
                let processes = process::list_processes()?;
                for process in processes {
                    println!("  {}", process);
                }
            }
            "attach" | "a" => {
                if parts.len() < 2 {
                    println!("Usage: attach <process_name>");
                    continue;
                }
                
                let process_name = parts[1];
                println!("Attaching to process: {}", process_name);
                match process::attach(process_name) {
                    Ok(process) => {
                        println!("Successfully attached to process: {}", process);
                        attached_process = Some(process);
                    }
                    Err(e) => {
                        println!("Failed to attach: {}", e);
                    }
                }
            }
            "scan" | "s" => {
                if attached_process.is_none() {
                    println!("Error: No process attached. Use 'attach' command first.");
                    continue;
                }
                
                if parts.len() < 2 {
                    println!("Usage: scan <value> [--fuzzy <tolerance>]");
                    continue;
                }
                
                let value = parts[1];
                let process = attached_process.as_ref().unwrap();
                println!("Scanning for value: {} in process {}", value, process);
                
                // Check for fuzzy flag
                let fuzzy_index = parts.iter().position(|&x| x == "--fuzzy" || x == "-f");
                let results = if let Some(index) = fuzzy_index {
                    if index + 1 < parts.len() {
                        let tolerance: f32 = parts[index + 1].parse().unwrap_or(1.0);
                        scanner::scan_fuzzy(process, value, tolerance)?
                    } else {
                        println!("Usage: scan <value> --fuzzy <tolerance>");
                        continue;
                    }
                } else {
                    scanner::scan_exact(process, value)?
                };
                
                if results.is_empty() {
                    println!("No results found.");
                } else {
                    println!("Found {} result(s):", results.len());
                    for result in results {
                        println!("  Address: 0x{:x}, Value: {}", result.address, result.value);
                    }
                }
            }
            "modify" | "m" => {
                if attached_process.is_none() {
                    println!("Error: No process attached. Use 'attach' command first.");
                    continue;
                }
                
                if parts.len() < 3 {
                    println!("Usage: modify <address> <new_value>");
                    continue;
                }
                
                let address = parts[1];
                let new_value = parts[2];
                let process = attached_process.as_ref().unwrap();
                println!("Modifying address {} to value {} in process {}", address, new_value, process);
                
                // Parse the address
                let addr = if address.starts_with("0x") {
                    usize::from_str_radix(&address[2..], 16)?
                } else {
                    address.parse::<usize>()?
                };
                
                // Convert the new value to bytes
                let value_bytes = if new_value.starts_with("0x") {
                    // Hexadecimal
                    let val = u32::from_str_radix(&new_value[2..], 16)?;
                    val.to_le_bytes().to_vec()
                } else if new_value.contains('.') {
                    // Float
                    let val = new_value.parse::<f32>()?;
                    val.to_le_bytes().to_vec()
                } else {
                    // Decimal
                    let val = new_value.parse::<u32>()?;
                    val.to_le_bytes().to_vec()
                };
                
                // Write to memory
                match memory::write_memory(process, addr, &value_bytes) {
                    Ok(()) => {
                        println!("Successfully wrote {} bytes to address 0x{:x}", value_bytes.len(), addr);
                    }
                    Err(e) => {
                        println!("Failed to write memory: {}", e);
                    }
                }
            }
            _ => {
                println!("Unknown command: '{}'. Type 'help' for available commands.", command);
            }
        }
    }
    
    Ok(())
}
