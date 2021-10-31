#![allow(unused_variables)]
use async_trait::async_trait;
use futures_core::stream::BoxStream;
use stdio::StdioData;
use tf::provider_server::Provider;

pub mod tf {
    use tonic::include_proto;
    include_proto!("tfplugin6");
}

#[derive(Debug, Default)]
pub struct HelloWorldProvider {}

#[async_trait]
impl Provider for HelloWorldProvider {
    async fn get_provider_schema(
        &self,
        request: tonic::Request<tf::get_provider_schema::Request>,
    ) -> Result<tonic::Response<tf::get_provider_schema::Response>, tonic::Status> {
        unimplemented!();
    }
    async fn validate_provider_config(
        &self,
        request: tonic::Request<tf::validate_provider_config::Request>,
    ) -> Result<tonic::Response<tf::validate_provider_config::Response>, tonic::Status> {
        unimplemented!();
    }
    async fn validate_resource_config(
        &self,
        request: tonic::Request<tf::validate_resource_config::Request>,
    ) -> Result<tonic::Response<tf::validate_resource_config::Response>, tonic::Status> {
        unimplemented!();
    }
    async fn validate_data_resource_config(
        &self,
        request: tonic::Request<tf::validate_data_resource_config::Request>,
    ) -> Result<tonic::Response<tf::validate_data_resource_config::Response>, tonic::Status> {
        unimplemented!();
    }
    async fn upgrade_resource_state(
        &self,
        request: tonic::Request<tf::upgrade_resource_state::Request>,
    ) -> Result<tonic::Response<tf::upgrade_resource_state::Response>, tonic::Status> {
        unimplemented!();
    }

    async fn configure_provider(
        &self,
        request: tonic::Request<tf::configure_provider::Request>,
    ) -> Result<tonic::Response<tf::configure_provider::Response>, tonic::Status> {
        unimplemented!();
    }
    async fn read_resource(
        &self,
        request: tonic::Request<tf::read_resource::Request>,
    ) -> Result<tonic::Response<tf::read_resource::Response>, tonic::Status> {
        unimplemented!();
    }
    async fn plan_resource_change(
        &self,
        request: tonic::Request<tf::plan_resource_change::Request>,
    ) -> Result<tonic::Response<tf::plan_resource_change::Response>, tonic::Status> {
        unimplemented!();
    }
    async fn apply_resource_change(
        &self,
        request: tonic::Request<tf::apply_resource_change::Request>,
    ) -> Result<tonic::Response<tf::apply_resource_change::Response>, tonic::Status> {
        unimplemented!();
    }
    async fn import_resource_state(
        &self,
        request: tonic::Request<tf::import_resource_state::Request>,
    ) -> Result<tonic::Response<tf::import_resource_state::Response>, tonic::Status> {
        unimplemented!();
    }
    async fn read_data_source(
        &self,
        request: tonic::Request<tf::read_data_source::Request>,
    ) -> Result<tonic::Response<tf::read_data_source::Response>, tonic::Status> {
        unimplemented!();
    }
    async fn stop_provider(
        &self,
        request: tonic::Request<tf::stop_provider::Request>,
    ) -> Result<tonic::Response<tf::stop_provider::Response>, tonic::Status> {
        unimplemented!();
    }
}

pub mod stdio {
    use tonic::include_proto;
    include_proto!("plugin");
}

#[derive(Debug, Default)]
pub struct StdioProvider {}

#[async_trait]
impl stdio::grpc_stdio_server::GrpcStdio for StdioProvider {
    type StreamStdioStream = BoxStream<'static, Result<StdioData, tonic::Status>>;

    async fn stream_stdio(
        &self,
        request: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::StreamStdioStream>, tonic::Status> {
        unimplemented!();
    }
}
