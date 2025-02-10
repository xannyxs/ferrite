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

    # Other
    grub2
    gdb
    bear
  ];

  shellHook = ''
    rustup default nightly
    rustup component add rust-src
    echo "Development environment ready!"
  '';
}
