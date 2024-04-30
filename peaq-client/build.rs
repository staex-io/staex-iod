use std::env;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::Config::new()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["src/peaq_did.proto"], &["src/"])?;
    if !std::process::Command::new("cp")
        .args(vec![
            &format!("{}/document.rs", env::var("OUT_DIR")?),
            "src/document.rs",
        ])
        .output()?
        .status
        .success()
    {
        return Err("failed to copy generated protobuf to src folder".to_string().into());
    }
    Ok(())
}
