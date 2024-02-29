{
  description = "Rapidly Assembled Ferris Fortune Locator Engine";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      perSystem = {
        config,
        pkgs,
        system,
        lib,
        self',
        ...
      }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };

        packages = {
          rust-toolchain =
            pkgs.rust-bin.stable.latest.default;

          raffle = let
            rustPlatform = pkgs.makeRustPlatform {
              cargo = self'.packages.rust-toolchain;
              rustc = self'.packages.rust-toolchain;
            };
            cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          in
            rustPlatform.buildRustPackage {
              pname = cargoToml.package.name;
              version = cargoToml.package.version;
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
            };
        };

        devShells.default = pkgs.mkShell {
          packages = [
            config.packages.rust-toolchain
            pkgs.just
          ];
        };
      };
    };
}
