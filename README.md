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

verify 

```bash 
which rnvm
```

expected output: 
/usr/local/bin/rnvm

### Run RNVM once

```bash 
rnvm ls
```

this automatically creates below files in your hoem dir:

```bash 
~/.rnvm/
 ├── versions/
 ├── config.json
 └── init.sh
```

### RNVM needs shell integration to switch Node versions without manual eval.
Add the following line to your shell config.

RNVM needs shell integration to switch Node versions without manual eval.
Add the following line to your shell config.

For zsh (macOS default)
```bash 
echo 'source ~/.rnvm/init.sh' >> ~/.zshrc
source ~/.zshrc
```

For bash
```bash 
echo 'source ~/.rnvm/init.sh' >> ~/.zshrc
source ~/.zshrc
```

This step is mandatory. Without this rnvm wont be able to change node versions.

## Usage

### Install a node version

```bash 
rnvm install 18.19.0
```

### List installed Node versions

```bash 
rnvm ls
```

### Switch to a node version

```bash 
rnvm use 18.9.0
```

### Verify

```bash 
node --version
```

### Show current node executable

```bash
rnvm current
```

### Uninstall a node version

```bash 
rnvm uninstall 18.9.0
```

### directory layout 

```bash 
~/.rnvm/
 ├── versions/
 │    ├── v18.19.0/
 │    ├── v20.11.0/
 ├── config.json
 └── init.sh
```
