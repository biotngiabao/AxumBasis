use crate::common::proto;
use crate::common::proto::hera_x_service_client::*;
use anyhow::Ok;
use tokio::stream;
use tonic::transport::Channel;

pub struct GprcClient {
    pub client: HeraXServiceClient<Channel>,
}

impl GprcClient {
    pub async fn connect(host: String, port: u16) -> anyhow::Result<Self> {
        let url = format!("http://{host}:{port}");
        let client: HeraXServiceClient<Channel> = HeraXServiceClient::connect(url.clone())
            .await?
            .accept_compressed(tonic::codec::CompressionEncoding::Gzip);

        println!("Connect to grpc server at {url}");

        Ok(Self { client: client })
    }

    pub async fn get_server_info(&self) -> anyhow::Result<proto::ServerInfoData> {
        let mut client = self.client.clone();

        let request = tonic::Request::new(proto::GetServerInfoRequest {
            id: 12,
            version: "1.0.0".to_string(),
        });
        let response: proto::GetServerInfoResponse =
            client.get_server_info(request).await?.into_inner();

        let value = response
            .value
            .ok_or_else(|| anyhow::anyhow!("no value returned in GetServerInfoResponse"))?;

        let data = match value {
            proto::get_server_info_response::Value::Data(d) => d,
            proto::get_server_info_response::Value::Message(msg) => {
                return Err(anyhow::anyhow!("unexpected variant Message: {}", msg));
            }
        };

        return Ok(data);
    }

    pub async fn ping(&self) -> anyhow::Result<proto::ErrorMsg> {
        let mut client = self.client.clone();

        let request = tonic::Request::new({});
        let response: proto::ErrorMsg = client.ping(request).await?.into_inner();

        return Ok(response);
    }

    pub async fn pca(
        &self,
        file_path: String,
        stream: bool,
        n_components: i32,
    ) -> anyhow::Result<proto::dimred_pca_response::Value> {
        let mut client = self.client.clone();
        let request = tonic::Request::new(proto::DimredPcaRequest {
            file: Some(proto::FileRequest {
                file_path,
                stream,
                is_over_write: true,
            }),
            n_components,
            batch: "".to_string(),
            key_store: "X_pca".to_string(),
        });

        let response = client.dimred_pca(request).await?.into_inner();
        let value = response
            .value
            .ok_or_else(|| anyhow::anyhow!("no value returned in pca"))?;

        return Ok(value);
    }
}
