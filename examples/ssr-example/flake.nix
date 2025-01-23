{
  description = "A flake for building the leptos leaflet hotline project";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
    in
    {

      # packages that can be run for development:
      #
      #   $ nix develop
      #
      devShells.default = pkgs.mkShell {

        buildInputs =
          with pkgs;
          [
            clang
            llvmPackages.bintools
            rustup
            wasm-pack
            openssl
            pkg-config
            cacert
            cargo-make
            trunk
            sass
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
              targets = [ "wasm32-unknown-unknown" ];
            }))
          ];

        shellHook = ''
          rustup toolchain install nightly --allow-downgrade
          rustup default nightly
          rustup target add wasm32-unknown-unknown
          cargo install cargo-generate cargo-leptos@0.2.24 wasm-pack
          cargo leptos watch
        '';
      };

    }
    );
}
