# RNVM
node version manager from scratch in rust

## Requirements

- macOS (Intel or Apple Silicon)
- Rust (stable)


## Commands

- install - Installs a node verison
- ls - lists all the node binaries installed
- current - prints current node executable
- use - switches to a specific node version
- uninstall - uninstalls a node version

### Build RVNM

```bash 
cargo build --release
```
This creates the binary at: 

target/release/rnvm

### RNVM must me available on your PATH to execute shell commands

```bash
sudo cp target/release/rnvm /usr/local/bin/rnvm
```

