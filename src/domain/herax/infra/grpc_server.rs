use crate::common::proto::hera_x_service_client::HeraXServiceClient;
use crate::common::proto::hera_x_service_server::{HeraXService, HeraXServiceServer};

use crate::common::proto::{self, *};
use crate::domain::herax::infra::grpc_client::GprcClient;
use tonic::transport::Channel;
use tonic::{Request, Response, Status, client};

pub struct HeraXServiceImpl {
    pub client: GprcClient,
}

#[tonic::async_trait]
impl HeraXService for HeraXServiceImpl {
    async fn ping(&self, request: Request<()>) -> Result<Response<ErrorMsg>, Status> {
        let mut client = self.client.client.clone();
        return client.ping({}).await;
    }
    async fn get_server_info(
        &self,
        request: Request<GetServerInfoRequest>,
    ) -> Result<Response<GetServerInfoResponse>, Status> {
        unimplemented!()
    }
    async fn anndata_info(
        &self,
        request: Request<AnndataInfoRequest>,
    ) -> Result<Response<AnndataInfoResponse>, Status> {
        unimplemented!()
    }

    async fn dimred_pca(
        &self,
        request: Request<DimredPcaRequest>,
    ) -> Result<Response<DimredPcaResponse>, Status> {
        let start = std::time::Instant::now();

        let mut client = self.client.client.clone();

        let resp = client.dimred_pca(request).await?;

        let duration = start.elapsed();
        println!("dimred_pca took {:?}", duration);

        Ok(resp)
    }
}
