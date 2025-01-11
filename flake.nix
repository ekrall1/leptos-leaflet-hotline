{
  description = "A flake for building the leptos leaflet hotline project";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; };
      overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
    in
    {

      # packages that can be run for development:
      #
      #   $ nix develop
      #
      devShells.${pkgs.system}.default = pkgs.mkShell {
        buildInputs =
          with pkgs;
          [
            clang
            llvmPackages.bintools
            rustup
            wasm-pack
            chromedriver
          ];

        shellHook = ''
          echo "Entering development shell"
        '';
      };

      };
}
