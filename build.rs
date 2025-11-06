use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let our_dir = PathBuf::from(env::var("OUT_DIR")?);

    tonic_build::configure()
        .file_descriptor_set_path(our_dir.join("api.bin"))
        .compile_protos(&["proto/api.proto"], &["proto"])?;

    Ok(())
}
