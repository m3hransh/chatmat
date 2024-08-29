use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve the environment variable
    let proto_path = env::var("PROTO_FILE_PATH").expect("PROTO_FILE_PATH not set");

    // Use the environment variable in tonic_build
    tonic_build::compile_protos(&proto_path).expect("Failed to compile proto files");

    // If you want Cargo to re-run the build script when the proto file changes:
    println!("cargo:rerun-if-changed={}", proto_path);
    Ok(())
}
