{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    devenv.url = "github:cachix/devenv";
  };

  outputs = { self, nixpkgs, devenv, flake-utils, ... } @ inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        cargo-pretty-test = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cargo-pretty-test";
          version = "v0.2.3";
          src = pkgs.fetchFromGitHub {
            owner = "josecelano";
            repo = "cargo-pretty-test";
            rev = "main";
            hash = "sha256-VnnhSgvNfqXLKTYe+Sef9H80+Ym7BBo7Jnfd2eMWF4U=";
          };
          cargoLock = {
            lockFile = src + "/Cargo.lock";
          };
          doCheck = false;
        };
        
        pname = "onano";
        version = "0.1.0";
      in
      {
        devShells.default = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            {
              languages.rust.enable = true;
              packages = [ cargo-pretty-test ];
            }
          ];
        };
        packages.default = pkgs.rustPlatform.buildRustPackage {
          inherit pname version;
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
      });
}
