use std::io::{self, Write};
use std::thread;
use std::time::Duration;

// A simple struct to hold our test values
struct GameState {
    health: u32,
    score: u32,
    position: f32,
}

fn main() -> io::Result<()> {
    println!("Test Application for Cheat Engine");
    println!("=================================");
    
    // Create our game state
    let mut game_state = GameState {
        health: 100,
        score: 0,
        position: 10.5,
    };
    
    println!("Initial values:");
    println!("  Health: {}", game_state.health);
    println!("  Score: {}", game_state.score);
    println!("  Position: {}", game_state.position);
    println!();
    
    // Print the memory addresses of our values
    println!("Memory addresses (for reference):");
    println!("  Health address: {:p}", &game_state.health);
    println!("  Score address: {:p}", &game_state.score);
    println!("  Position address: {:p}", &game_state.position);
    println!();
    
    println!("This application will run for 60 seconds.");
    println!("Use the cheat engine to modify the values while it's running.");
    println!("Press Ctrl+C to exit early.");
    println!();
    
    // Run for 60 seconds, printing the values every second
    for i in 1..=60 {
        // Update values slightly each second to make it more interesting
        game_state.score += 1;
        game_state.position += 0.1;
        
        println!("Second {}: Health={}, Score={}, Position={}", 
                 i, game_state.health, game_state.score, game_state.position);
        
        // Flush stdout to ensure output is visible immediately
        io::stdout().flush()?;
        
        // Sleep for 1 second
        thread::sleep(Duration::from_secs(1));
    }
    
    println!("Test application finished.");
    Ok(())
}