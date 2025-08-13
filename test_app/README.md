# Test Application for Cheat Engine

This is a simple test application that you can use to test the cheated_cheat_engine.

## What it does

The application creates a simple game state with three values:
- Health (u32): Starts at 100
- Score (u32): Starts at 0 and increments by 1 each second
- Position (f32): Starts at 10.5 and increments by 0.1 each second

The application runs for 60 seconds, printing the current values each second.

## How to use it for testing

1. Build and run the test application:
   ```bash
   cargo run
   ```

2. Note the memory addresses printed at startup (these will be different each time you run the application)

3. In another terminal, use the cheated_cheat_engine to attach to the test_app process and modify its values:
   ```bash
   # Start interactive mode
   ../target/release/cheated_cheat_engine interactive
   
   # In interactive mode:
   # > list  # Find the test_app process
   # > attach test_app
   # > scan 100  # Find the health value
   # > modify <address> <new_value>  # Modify a value
   ```

## Example workflow

1. Start the test application:
   ```bash
   cargo run
   ```
   Output:
   ```
   Test Application for Cheat Engine
   =================================
   Initial values:
     Health: 100
     Score: 0
     Position: 10.5

   Memory addresses (for reference):
     Health address: 0x7fff5fbff6ac
     Score address: 0x7fff5fbff6b0
     Position address: 0x7fff5fbff6b4

   This application will run for 60 seconds.
   Use the cheat engine to modify the values while it's running.
   Press Ctrl+C to exit early.

   Second 1: Health=100, Score=1, Position=10.6
   Second 2: Health=100, Score=2, Position=10.7
   ...
   ```

2. Use the cheat engine to find and modify values:
   ```bash
   ../target/release/cheated_cheat_engine interactive
   ```
   In interactive mode:
   ```
   > list
   Listing running processes...
     test_app (PID: 12345)
   > attach test_app
   Attaching to process: test_app
   Successfully attached to process: test_app (PID: 12345)
   > scan 100
   Scanning for value: 100 in process test_app (PID: 12345)
   Found 1 result(s):
     Address: 0x7fff5fbff6ac, Value: 100
   > modify 0x7fff5fbff6ac 999
   Modifying address 0x7fff5fbff6ac to value 999 in process test_app (PID: 12345)
   Successfully wrote 4 bytes to address 0x7fff5fbff6ac
   ```

3. You should see the modified value in the test application output:
   ```
   Second 5: Health=999, Score=5, Position=11.0
   ```

## Notes

- The memory addresses will be different each time you run the application
- On Unix-like systems, you may need to run the cheat engine with `sudo` to access memory of other processes
- This is for educational purposes only