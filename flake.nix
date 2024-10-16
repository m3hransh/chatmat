{
  description = "A Nix-flake-based Go development environment";

  # GitHub URLs for the Nix inputs we're using
  inputs = {
    # Simply the greatest package repository on the planet
    nixpkgs.url = "github:NixOS/nixpkgs";
    # A set of helper functions for using flakes
    flake-utils.url = "github:numtide/flake-utils";
    # A utility library for working with Rust
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          # This overlay adds the "rust-bin" package to nixpkgs
          (import rust-overlay)
        ];
        pkgs = import nixpkgs { inherit system overlays; };
        # Use the specific version of the Rust toolchain specified by the toolchain file
        localRust = pkgs.rust-bin.fromRustupToolchainFile ./chatmat-back/rust-toolchain.toml;

        node = pkgs.nodejs_latest;
        # Other utilities commonly used in Rust projects (but not in this example project)

      in
      {
        packages = {
          default = pkgs.callPackage ./. { inherit pkgs localRust; };
        };

        devShells.default =
          with pkgs; mkShell {
            # Packages included in the environment
            packages = [ envoy ];
            buildInputs = [
              node
              localRust
              openssl
              pkg-config
              protobuf
              grpcurl
              protoc-gen-js
              protoc-gen-grpc-web
              pnpm
            ];

            # Run when the shell is started up
            shellHook = ''
              echo "node `${node}/bin/node --version`"
              ${localRust}/bin/cargo --version
              export PROTOBUF_LOCATION="${pkgs.protobuf}"
              export PROTOC=$PROTOBUF_LOCATION/bin/protoc
              export PROTOC_INCLUDE=$PROTOBUF_LOCATION/include
              export PROTOC_JS="${protoc-gen-grpc-web}/bin/protoc-gen-grpc-web"
            '';
          };

      }
    );
}
