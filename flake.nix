{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    naersk = {
      url = "github:nix-community/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system}.appendOverlays [
          rust-overlay.overlays.default
        ];
        rustFromFile = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        nativeBuildInputs = [ ];
        naerskLib = pkgs.callPackage naersk {
          rustc = rustFromFile;
          cargo = rustFromFile;
        };
      in
      {
        defaultPackage = naerskLib.buildPackage {
          pname = "pest-example";
          root = ./.;
          nativeBuildInputs = nativeBuildInputs;
        };

        defaultApp = flake-utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
        };

        devShell = with pkgs; mkShell {
          nativeBuildInputs = [ rustFromFile ] ++ nativeBuildInputs;

          buildInputs = [
            cargo-audit
            cargo-edit
          ];

          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
