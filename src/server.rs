#![allow(unused_variables)]
use std::collections::HashMap;

use async_stream::try_stream;
use async_trait::async_trait;
use futures_core::stream::BoxStream;
use stdio::ConnInfo;
use stdio::StdioData;
use tf::provider_server::Provider;
use tf::StringKind;
use tokio::time::{sleep, Duration};

pub mod tf {
    use tonic::include_proto;
    include_proto!("tfplugin5");
}

#[derive(Debug, Default)]
pub struct HelloWorldProvider {}

#[async_trait]
impl Provider for HelloWorldProvider {
    async fn upgrade_resource_state(
        &self,
        request: tonic::Request<tf::upgrade_resource_state::Request>,
    ) -> Result<tonic::Response<tf::upgrade_resource_state::Response>, tonic::Status> {
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
    async fn get_schema(
        &self,
        request: tonic::Request<tf::get_provider_schema::Request>,
    ) -> Result<tonic::Response<tf::get_provider_schema::Response>, tonic::Status> {
        Ok(tonic::Response::new(tf::get_provider_schema::Response {
            provider: Some(tf::Schema {
                version: 1,
                block: Some(tf::schema::Block {
                    version: 1,
                    attributes: vec![],
                    block_types: vec![],
                    description: "registry.terraform.io/hashicorp/helloworld".to_string(),
                    description_kind: StringKind::Plain as i32,
                    deprecated: false,
                }),
            }),
            resource_schemas: HashMap::new(),
            data_source_schemas: HashMap::new(),
            diagnostics: vec![],
            provider_meta: Some(tf::Schema {
                version: 1,
                block: Some(tf::schema::Block {
                    version: 1,
                    attributes: vec![],
                    block_types: vec![],
                    description: "registry.terraform.io/hashicorp/helloworld".to_string(),
                    description_kind: 1,
                    deprecated: false,
                }),
            }),
        }))
    }
    async fn prepare_provider_config(
        &self,
        request: tonic::Request<tf::prepare_provider_config::Request>,
    ) -> Result<tonic::Response<tf::prepare_provider_config::Response>, tonic::Status> {
        unimplemented!();
    }
    async fn validate_resource_type_config(
        &self,
        request: tonic::Request<tf::validate_resource_type_config::Request>,
    ) -> Result<tonic::Response<tf::validate_resource_type_config::Response>, tonic::Status> {
        unimplemented!();
    }
    async fn validate_data_source_config(
        &self,
        request: tonic::Request<tf::validate_data_source_config::Request>,
    ) -> Result<tonic::Response<tf::validate_data_source_config::Response>, tonic::Status> {
        unimplemented!();
    }
    async fn configure(
        &self,
        request: tonic::Request<tf::configure::Request>,
    ) -> Result<tonic::Response<tf::configure::Response>, tonic::Status> {
        unimplemented!();
    }
    async fn stop(
        &self,
        request: tonic::Request<tf::stop::Request>,
    ) -> Result<tonic::Response<tf::stop::Response>, tonic::Status> {
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
        return Ok(tonic::Response::new(Box::pin(try_stream! {
            loop {
                yield StdioData{channel: 1, data: vec![]};
                sleep(Duration::from_secs(30)).await;
            }
        })));
    }
}

#[derive(Debug, Default)]
pub struct BrokerProvider {}

#[async_trait]
impl stdio::grpc_broker_server::GrpcBroker for BrokerProvider {
    type StartStreamStream = BoxStream<'static, Result<ConnInfo, tonic::Status>>;

    async fn start_stream(
        &self,
        request: tonic::Request<tonic::Streaming<ConnInfo>>,
    ) -> Result<tonic::Response<Self::StartStreamStream>, tonic::Status> {
        return Ok(tonic::Response::new(Box::pin(try_stream! {
            loop {
                yield ConnInfo{service_id: 1, network: String::from("network"), address: String::from("address")};
                sleep(Duration::from_secs(30)).await;
            }
        })));
    }
}
