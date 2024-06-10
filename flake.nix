{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };
        naersk' = pkgs.callPackage naersk { };
      in
      rec {
        defaultPackage = naersk'.buildPackage
          {
            src = ./.;
            buildInputs = with pkgs; [ openssl pkg-config ];
          };

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            openssl
            pkg-config
            clippy
            rust-analyzer
          ];
          nativeBuildInputs = with pkgs; [ rustc cargo ];
        };
      }
    );
}
