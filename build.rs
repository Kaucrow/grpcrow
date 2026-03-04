use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src").join("pb");
    fs::create_dir_all(&out_dir).unwrap();

    tonic_prost_build::configure()
        .out_dir(out_dir)
        .compile_protos(
            &["proto/read.proto"], 
            &["proto"]
        )?;

    Ok(())
}