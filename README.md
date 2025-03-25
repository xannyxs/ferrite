# Fungul

Fungul is an experimental kernel written in Rust. Just as fungal networks in nature create vast, efficient systems for resource sharing, Fungul aims to provide a robust foundation for process communication and resource management.

> This project is in early stage development. While it's exciting to experiment with, it's not yet ready for production use.

## Building from Source

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

## Documentation

Comprehensive documentation is available in the `/docs` directory. To explore the documentation:

```bash
mdbook serve --open docs
```

The documentation provides insights into Fungul's design philosophy and implementation details, which could be valuable for your own kernel development journey. Take this with a grain of salt, since I am not fimiliar with everything either.

## Why "Fungul"?

The name "Fungul" draws inspiration from both Unix naming conventions and natural fungal networks. Like the mycorrhizal networks that connect forest ecosystems, Fungul aims to create an efficient, interconnected system for resource sharing and process communication. This biological metaphor extends to our architecture, where we treat system components as nodes in a living, adapting network.

## License

Fungul is licensed under the GPL License. See [LICENSE](LICENSE) for details.
