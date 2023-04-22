{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
    cargo2nix.url    = "github:tenx-tech/cargo2nix";
    #ravedude.url = "github:Rahix/avr-hal/ravedude";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, cargo2nix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        #my-rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
        #  extensions = [ "rust-src" ];
        #  targets = [ "thumbv7em-none-eabihf" "arm-unknown-linux-gnueabihf" ];
        #});
        my-rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        #my-rust = pkgs.rust-bin.stable.latest.default.override {
        #  extensions = [ "rust-src" ];
        #  targets = [ "thumbv7em-none-eabihf" "arm-unknown-linux-gnueabihf" ];
        #};
        #ravedude = pkgs.callPackage ravedude {
        #  src = builtins.fetchGit {
        #    url = "https://github.com/Rahix/avr-hal";
        #    rev = "fb609ab0d14c5a0a44e2dff3e5e514cb612a529a";
        #  };
        #};
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            exa
            fd
            #rust-nightly
            #rust-bin.stable.latest.default
            my-rust
            rustup
            rust-analyzer
            cargo-generate
            cargo-embed
            cargo-binutils
            libusb
            stdenv.cc.cc.lib
            zlib
            glib
            arduino-cli
            arduino-core
            unzip
            patchelf
            avrdude
            gnumake
            glibc
            gcc
            pkgsCross.avr.buildPackages.gcc
            gdb
            picocom
            minicom
            gcc-arm-embedded
            #ravedude
            #(pkgs.callPackage (cargo2nix.defaultPackageBuilder { packageFun = import cargo2nix.lib; }) { src = builtins.fetchGit { url = "https://github.com/Rahix/avr-hal"; rev = "fb609ab0d14c5a0a44e2dff3e5e514cb612a529a"; }; })
          ];

          shellHook = ''
            #alias ls=exa
            #alias find=fd
            rustup run stable cargo install ravedude --force
            #rustup component add llvm-tools-preview
            export PATH="$HOME/.cargo/bin:$PATH"
            export RAVEDUDE_PORT=/dev/ttyUSB0
          '';
        };
      }
    );
}
    #= help: consider downloading the target with `rustup target add avr-atmega328p`
