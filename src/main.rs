mod server;
use std::env;

use anyhow::{anyhow, Result};
use futures::{try_join, TryFutureExt};
use rcgen::{BasicConstraints, IsCa};
use server::tf::provider_server::ProviderServer;
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};
use tracing_subscriber::fmt::format;

use server::stdio::grpc_stdio_server::GrpcStdioServer;


const CORE_PROTOCOL_VERSION: u8 = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let collector = tracing_subscriber::fmt()
        .event_format(format().json())
        .finish();
    tracing::subscriber::set_global_default(collector).unwrap();

    let addr = "0.0.0.0:10000".parse()?;
    let hello_world = server::HelloWorldProvider::default();
    let stdio = server::StdioProvider::default();

    let client_cert = Certificate::from_pem(
        env::var("PLUGIN_CLIENT_CERT").expect("Expected to find cert in PLUGIN_CLIENT_CERT"),
    );
    let mut cp = rcgen::CertificateParams::new(vec!["localhost".to_string()]);
    cp.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    let server_cert = rcgen::Certificate::from_params(cp)?;
    let identity = Identity::from_pem(
        server_cert.serialize_pem()?,
        server_cert.serialize_private_key_pem(),
    );
    let mut tls_config = ServerTlsConfig::new();
    tls_config = tls_config.client_ca_root(client_cert);
    tls_config = tls_config.identity(identity);

    let serve = Server::builder()
        .tls_config(tls_config)
        .unwrap()
        .add_service(ProviderServer::new(hello_world))
        .add_service(GrpcStdioServer::new(stdio))
        .serve(addr);

    async fn info(server_cert: rcgen::Certificate) -> Result<()> {
        println!(
            "{}|{}|{}|{}|{}|{}",
            CORE_PROTOCOL_VERSION,
            "5",
            "tcp",
            "localhost:10000",
            "grpc",
            base64::encode_config(
                server_cert.serialize_der().unwrap(),
                base64::STANDARD_NO_PAD
            )
        );
        Ok(())
    }

    try_join!(serve.map_err(|e| anyhow!(e)), info(server_cert))?;

    Ok(())
}
