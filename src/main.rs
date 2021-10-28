mod server;
use tonic::{transport::Server};
use server::tf::provider_server::ProviderServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let hello_world = server::HelloWorldProvider::default();

    Server::builder()
        .add_service(ProviderServer::new(hello_world))
        .serve(addr)
        .await?;

    Ok(())
}