{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
  ];
  buildInputs = [
    clang
    llvmPackages.bintools
    fontconfig
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
