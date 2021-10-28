fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("external/terraform/docs/plugin-protocol/tfplugin6.0.proto")?;
    Ok(())
}