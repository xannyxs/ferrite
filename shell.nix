{
  pkgs ? import <nixpkgs> { },
}:
let
  pkgs = import <nixpkgs> {
    crossSystem = {
      config = "i386-elf";
      system = "i686-unknown-none";
    };
  };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    gcc
    binutils
    gnumake
    nasm
    grub2
    xorriso
  ];

  shellHook = ''
    		export TARGET=i386-elf
    		echo "OS Development Environment Ready"
    		echo "Target architecture: $TARGET"
    		'';
}
