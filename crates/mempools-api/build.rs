fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("./src")
        .file_descriptor_set_path("./src/descriptor.bin")
        .compile(&["proto/api.proto"], &["proto"])?;
    Ok(())
}
