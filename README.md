# archie-rs
## Rust implementation of [archie](https://github.com/Helix128/archie).

### Compatibility
This version introduces breaking changes compared to the Python version. While the task system remains the same, the rest of the modules are being rewritten from the ground up.

# About Archie
Archie is a command line tool with multiple features aimed at making it easier to do repetitive tasks, managing environment variables and viewing sytem info.

# Features
- [x] User defined tasks
- [x] System info modules
    - [x] Disks / Partitions (partial functionality, W.I.P)
    - [ ] CPU
    - [ ] GPU
    - [ ] RAM
    - [ ] Network
- [ ] Environment variable management

# Example usage
```bash
archie help

## Task module (task)
# List all tasks
archie task list

# Create new tasks
# Single command
archie task set update "sudo pacman -Syu"

# Multiple commands
archie task set test "echo PING" "echo PONG"

# This also works
archie task set "marco polo" "echo Marco" "echo Polo"

# Run a specific task
archie task run update
# same as
archie pls update

# This also works
archie pls "marco polo"

# Find Archie task file (for sharing or manual edits)
archie task locate

# List partitions and their info
archie system partitions

```
