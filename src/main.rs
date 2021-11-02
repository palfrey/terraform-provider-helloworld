mod server;
use std::{env, fs::File, io::SeekFrom, sync::Mutex};

use anyhow::{anyhow, Result};
use futures::{try_join, TryFutureExt};
use rcgen::{BasicConstraints, IsCa};
use rustls::{
    internal::pemfile, ClientCertVerified, HandshakeSignatureValid, ProtocolVersion, TLSError,
};
use server::tf::provider_server::ProviderServer;
use tokio::io::AsyncSeekExt;
use tonic::transport::{Server, ServerTlsConfig};
use tower_http::trace::TraceLayer;

use rustls::internal::msgs::handshake::DigitallySignedStruct;
use server::stdio::grpc_broker_server::GrpcBrokerServer;
use server::stdio::grpc_stdio_server::GrpcStdioServer;

const CORE_PROTOCOL_VERSION: u8 = 1;

struct CertVerifier {
    pub cert: Vec<u8>,
    pub root_store: rustls::RootCertStore,
}

impl rustls::ClientCertVerifier for CertVerifier {
    fn client_auth_root_subjects(
        &self,
        _sni: Option<&webpki::DNSName>,
    ) -> Option<rustls::DistinguishedNames> {
        Some(self.root_store.get_subjects())
    }

    fn verify_client_cert(
        &self,
        presented_certs: &[rustls::Certificate],
        _sni: Option<&webpki::DNSName>,
    ) -> Result<rustls::ClientCertVerified, TLSError> {
        if presented_certs.len() != 1 {
            return Err(TLSError::General(format!(
                "server sent {} certificates, expected one",
                presented_certs.len()
            )));
        }
        if presented_certs[0].0 != self.cert {
            return Err(TLSError::General(format!(
                "server certificates doesn't match ours"
            )));
        }
        Ok(ClientCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::Certificate,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, TLSError> {
        // It's a SHA-512 ECDSA, which webpki doesn't support (because they're not generally used in the web)
        // See also https://www.chromestatus.com/feature/5725838074970112
        return Ok(HandshakeSignatureValid::assertion());
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_file = File::create("../my_cool_trace.log")?;
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_writer(Mutex::new(log_file))
        // .with_writer(io::stderr)
        .init();

    // let collector = tracing_subscriber::fmt()
    //     .event_format(format().json())
    //     .finish();
    // tracing::subscriber::set_global_default(collector).unwrap();

    let addr = "0.0.0.0:10000".parse()?;
    let hello_world = server::HelloWorldProvider::default();
    let stdio = server::StdioProvider::default();
    let broker = server::BrokerProvider::default();

    let mut client_root_cert_store = rustls::RootCertStore::empty();

    if let Ok(pem) = env::var("PLUGIN_CLIENT_CERT") {
        let mut pem_buffer = std::io::Cursor::new(pem);
        client_root_cert_store
            .add_pem_file(&mut pem_buffer)
            .unwrap();
    }
    let mut cp = rcgen::CertificateParams::new(vec!["localhost".to_string()]);
    cp.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    let server_cert = rcgen::Certificate::from_params(cp)?;

    let mut cert_buffer = std::io::Cursor::new(server_cert.serialize_pem()?);
    let tls_cert = pemfile::certs(&mut cert_buffer).unwrap();

    let mut key_buffer = std::io::Cursor::new(server_cert.serialize_private_key_pem());
    let mut key = pemfile::pkcs8_private_keys(&mut key_buffer).unwrap();

    cert_buffer.seek(SeekFrom::Start(0)).await?;

    let env_cert = env::var("PLUGIN_CLIENT_CERT").unwrap();
    let raw_cert = env_cert.as_bytes();
    let x509_cert = x509_parser::pem::parse_x509_pem(&raw_cert)
        .unwrap()
        .1
        .clone();
    let mut server_config = rustls::ServerConfig::new(std::sync::Arc::new(CertVerifier {
        cert: x509_cert.contents,
        root_store: client_root_cert_store,
    }));
    server_config.set_single_cert(tls_cert, key.pop().unwrap())?;
    // server_config.set_protocols(&[Vec::from("h2")]);
    server_config.versions = vec![ProtocolVersion::TLSv1_2];
    let mut tls_config = ServerTlsConfig::new();
    // tls_config = tls_config.identity(identity);
    tls_config.rustls_server_config(server_config);

    let serve = Server::builder()
        .tls_config(tls_config)
        .unwrap()
        .layer(TraceLayer::new_for_grpc())
        .add_service(ProviderServer::new(hello_world))
        .add_service(GrpcStdioServer::new(stdio))
        .add_service(GrpcBrokerServer::new(broker))
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
