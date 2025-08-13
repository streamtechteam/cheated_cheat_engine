//! Example usage of the cheated_cheat_engine library
use cheated_cheat_engine::{process, scanner};

fn main() -> anyhow::Result<()> {
    // Create a mock process for demonstration
    let process = process::Process::new(1234, "example_game.exe".to_string());
    
    println!("Scanning for value 100 in process {}", process);
    
    // Perform an exact scan
    let results = scanner::scan_exact(&process, "100")?;
    
    if results.is_empty() {
        println!("No results found");
    } else {
        println!("Found {} result(s):", results.len());
        for result in results {
            println!("  Address: 0x{:x}, Value: {}", result.address, result.value);
        }
    }
    
    // Perform a fuzzy scan
    println!("\nPerforming fuzzy scan for value 100 with tolerance 1.0:");
    let fuzzy_results = scanner::scan_fuzzy(&process, "100", 1.0)?;
    
    if fuzzy_results.is_empty() {
        println!("No results found");
    } else {
        println!("Found {} result(s):", fuzzy_results.len());
        for result in fuzzy_results {
            println!("  Address: 0x{:x}, Value: {}", result.address, result.value);
        }
    }
    
    Ok(())
}
