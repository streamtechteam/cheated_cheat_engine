# Cheated Cheat Engine

A simple cheat engine clone written in Rust.

## Features

- Process listing (shows actual running processes on the system)
- Memory scanning for exact values
- Fuzzy matching for floating-point values
- Memory modification
- Interactive mode for continuous workflow

## Installation

Make sure you have Rust and Cargo installed. Then clone this repository and build the project:

```bash
git clone https://github.com/streamtechteam/cheated_cheat_engine.git
cd cheated_cheat_engine
cargo build --release
```

## Usage

### Interactive Mode (Recommended)

For the best experience, use the interactive mode which allows you to attach to a process and perform multiple operations in the same session:

```bash
# Start interactive mode
./cheated_cheat_engine interactive

# In interactive mode, you can use these commands:
# > list                    # List running processes
# > attach <process_name>   # Attach to a process
# > scan <value>            # Scan for a specific value
# > scan <value> --fuzzy <tolerance>  # Fuzzy scan
# > modify <address> <new_value>      # Modify a value
# > help                    # Show help
# > exit                    # Exit interactive mode
```

### Command Mode

You can also use individual commands, but note that each command runs in a separate process:

```bash
# List running processes
./cheated_cheat_engine list

# Attach to a process (shows how to continue in interactive mode)
./cheated_cheat_engine attach process_name
```

## Interactive Mode Commands

- `list` or `l` - List running processes
- `attach <process_name>` or `a <process_name>` - Attach to a process
- `scan <value>` or `s <value>` - Scan for a specific value in the attached process
- `scan <value> --fuzzy <tolerance>` - Fuzzy scan for a value with tolerance
- `modify <address> <new_value>` or `m <address> <new_value>` - Modify a value at an address
- `help` or `h` - Show help
- `exit` or `quit` or `q` - Exit interactive mode

## Examples

### Interactive Session Example

```bash
$ ./cheated_cheat_engine interactive
> list
Listing running processes...
  systemd (PID: 1)
  my_game (PID: 12345)
> attach my_game
Attaching to process: my_game
Successfully attached to process: my_game (PID: 12345)
> scan 100
Scanning for value: 100 in process my_game (PID: 12345)
Found 2 result(s):
  Address: 0x555555555000, Value: 100
  Address: 0x555555556000, Value: 100
> modify 0x555555555000 999
Modifying address 0x555555555000 to value 999 in process my_game (PID: 12345)
Successfully wrote 4 bytes to address 0x555555555000
```

## Test Application

We've included a simple test application that you can use to verify the cheat engine works correctly:

```bash
# Build and run the test application
cd test_app
cargo run

# In another terminal, use the cheat engine to modify its values
../cheated_cheat_engine interactive
```

See `test_app/README.md` for detailed instructions on how to use the test application.

## Platform Support

This implementation works on both Windows and Unix-like systems (Linux, macOS):
- On Windows, it uses Windows API to enumerate processes and read/write memory
- On Unix-like systems, it reads from the `/proc` filesystem and uses `process_vm_readv`/`process_vm_writev`

## Important Notes

1. **Permissions**: On Unix-like systems, you may need to run with `sudo` to access memory of other processes
2. **Process Security**: Some processes may have memory protection that prevents reading or writing
3. **Address Validity**: Memory addresses can change between runs of a program

## Limitations

This is a simplified implementation for educational purposes. A real cheat engine would need:

- More sophisticated scanning algorithms
- A graphical user interface
- Additional data type support

## License

This project is licensed under the MIT License.