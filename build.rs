fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &[
            "external/terraform/docs/plugin-protocol/tfplugin5.0.proto",
            "external/go-plugin/internal/plugin/grpc_stdio.proto",
        ],
        &[
            "external/terraform/docs/plugin-protocol",
            "external/go-plugin/internal/plugin",
        ],
    )?;
    Ok(())
}
