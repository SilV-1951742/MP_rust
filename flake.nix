{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-utils, naersk, nixpkgs, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };
        
        naersk' = pkgs.callPackage naersk {
          cargo = fenix.packages.${system}.complete.toolchain;
          rustc = fenix.packages.${system}.complete.toolchain;
        };
        
      in rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
        };

        # For `nix develop` (optional, can be skipped):
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            pkg-config
            openssl
          ];
          nativeBuildInputs = with pkgs; [
            fenix.packages.${system}.complete.toolchain
          ];
        };
      }
    );
}
