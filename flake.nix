{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    {
      self,
      flake-utils,
      naersk,
      treefmt-nix,
      nixpkgs,
      advisory-db,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk { };
        treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
      in
      {
        packages.default = naersk'.buildPackage {
          src = ./.;
        };

        formatter = treefmtEval.config.build.wrapper;

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            rust-analyzer
            clippy
            cargo-audit
          ];
        };

        checks = {
          package = self.packages.${system}.default;

          formatter = treefmtEval.config.build.check self;

          cargo-audit = pkgs.stdenv.mkDerivation {
            name = "cargo-audit-check";
            src = ./.;

            nativeBuildInputs = with pkgs; [
              cargo
              cargo-audit
              cacert
            ];

            buildPhase = ''
              export HOME=$(mktemp -d)

              cargo audit --no-yanked --no-fetch --db ${advisory-db}
            '';

            installPhase = "touch $out";
          };
        };
      }
    );
}
