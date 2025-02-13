# Building from Source

You'll need:

- `nix-shell` for an isolated development environment
- QEMU for testing

```bash
# Clone the repository
git clone https://github.com/xannyxs/fungul
cd fungul

# Initiate nix-shell
nix-shell shell.nix --command "zsh"

# Build the kernel
make

# Run in QEMU
make run
```
