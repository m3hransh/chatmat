{ pkgs ? import <nixpkgs> { }, localRust }:
let
  protofile = pkgs.stdenv.mkDerivation {
    name = "protofile";
    src = ./proto;
    installPhase = ''
      mkdir $out
      cp chat.proto $out
    '';
  };
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = "chatmat-back";
  version = "0.1";
  buildInputs = [
    pkgs.protobuf
    pkgs.pkg-config
    pkgs.openssl
    protofile
  ];
  nativeBuildInputs = [
    localRust
  ];
  buildPhase = ''
    echo "Copying proto files..."
    ls ${protofile} 
    cp ${protofile}/chat.proto $TMPDIR

    # Export the path to the build.rs script
    export PROTO_FILE_PATH="$TMPDIR/chat.proto"
  '';

  installPhase = ''
    mkdir -p $out/bin
    cargo build --release
    cp target/release/chatmat-server $out/bin

  '';
  PROTOBUF_LOCATION = "${pkgs.protobuf}";
  PROTOC = "${pkgs.protobuf}/bin/protoc";
  PROTOC_INCLUDE = "${pkgs.protobuf}/include";
  cargoLock.lockFile = ./chatmat-back/Cargo.lock;
  src = pkgs.lib.cleanSource ./chatmat-back;
  RUST_BACKTRACE = 1;
}
