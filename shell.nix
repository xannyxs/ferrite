{ }:

let
  pinnedPkgs =
    import (builtins.fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-24.11.tar.gz")
      { };

  crossToolchain = pinnedPkgs.pkgsCross.i686-embedded.buildPackages.gcc;

in
pinnedPkgs.mkShell {
  buildInputs = with pinnedPkgs; [
    crossToolchain
    nasm
    gdb
    gnumake
    binutils
    xorriso

    # Rust specific
    pkg-config
    rustup
    clippy
    mdbook

    # Formatters
    markdownlint-cli
    alejandra

    # Other
    grub2
    bear
    tree
  ];

  shellHook = ''
    echo "✅ Development environment from nixpkgs 23.11 is ready!"
  '';
}
