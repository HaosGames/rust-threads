{
  description = "Lightning Network Protocol Node";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, rust-overlay, ... }:
    let
      name = "rust-threads";
    in
      utils.lib.eachSystem [ "x86_64-linux" ]
        (system:
        let
          # Imports
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              rust-overlay.overlay
              (final: prev: {
                # Because rust-overlay bundles multiple rust packages into one
                # derivation, specify that mega-bundle here, so that crate2nix
                # will use them automatically.
                rustc = final.rust-bin.stable.latest.default;
                cargo = final.rust-bin.stable.latest.default;
              })
            ];
          };

          # Configuration for the non-Rust dependencies
          buildInputs = with pkgs; [ openssl ];
          nativeBuildInputs = with pkgs; [ rustc cargo pkgconfig cmake ];
          buildEnvVars = {
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          };
          rust-project = pkgs.rustPlatform.buildRustPackage rec {
            pname = name;
            version = "0.0.1";
            src = self;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
            inherit nativeBuildInputs;
            inherit buildInputs;
          };
        in
            rec {
               packages.${name} = rust-project;

              # `nix build`
              defaultPackage = packages.${name};

              # `nix run`
              apps.${name} = utils.lib.mkApp {
                inherit name;
                drv = packages.${name};
              };
              defaultApp = apps.${name};

              # `nix develop`
              devShell = pkgs.mkShell
                {
                  inherit buildInputs nativeBuildInputs;
                  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
                } // buildEnvVars;
            }
        );
}
