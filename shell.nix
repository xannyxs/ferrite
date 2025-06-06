{
  pkgs ? import <nixpkgs> { },
}:

let
  crossToolchain = pkgs.pkgsCross.i686-embedded.buildPackages.gcc;
in
pkgs.mkShell {
  buildInputs = with pkgs; [
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
    echo "Development environment ready!"
  '';
}
