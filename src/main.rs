mod server;
use tonic::{transport::Server};
use server::tf::provider_server::ProviderServer;
use futures::{TryFutureExt, try_join};
use anyhow::{anyhow,Result};

const CORE_PROTOCOL_VERSION: u8 = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:10000".parse()?;
    let hello_world = server::HelloWorldProvider::default();

    let serve = Server::builder()
        .add_service(ProviderServer::new(hello_world))
        .serve(addr);

    async fn info() -> Result<()> {
        println!("{}|{}|{}|{}|{}|{}",
        CORE_PROTOCOL_VERSION,
        "5",
        "tcp",
        "localhost:10000",
        "grpc",
        "cert");
        Ok(())
    }

    try_join!(serve.map_err(|e| anyhow!(e)), info())?;

    Ok(())
}