use tonic::{
    Request, Response, Result,
    transport::{Channel, Endpoint},
};

use crate::rms;

use rms::gsc_gateway_api_client::GscGatewayApiClient;
use rms::rms_config_api_client::RmsConfigApiClient;

use tinc::well_known::prost::Empty;

pub struct RmsApiGateway {
    endpoint: Endpoint,
}

impl RmsApiGateway {
    pub async fn new(target: &str) -> Self {
        Self {
            endpoint: Endpoint::from_shared(target.to_string())
                .expect("Invalid endpoint URL in RmsApiGateway::new"),
        }
    }

    // create a client to connect to the target server
    fn client(&self) -> RmsConfigApiClient<Channel> {
        let conn = self.endpoint.connect_lazy();
        RmsConfigApiClient::new(conn)
    }
}

#[tonic::async_trait]
impl rms::rms_config_api_server::RmsConfigApi for RmsApiGateway {
    async fn list_satellites(
        &self,
        req: Request<Empty>,
    ) -> Result<Response<rms::ListSatellitesResponse>> {
        self.client().list_satellites(req).await
    }

    async fn list_beams(
        &self,
        request: Request<rms::ListBeamsRequest>,
    ) -> Result<Response<rms::ListBeamsResponse>> {
        self.client().list_beams(request).await
    }

    async fn get_beam_config(
        &self,
        req: Request<rms::SatBeamId>,
    ) -> Result<Response<rms::BeamBasicConfig>> {
        self.client().get_beam_config(req).await
    }

    async fn set_beam_config(&self, req: Request<rms::BeamBasicConfig>) -> Result<Response<Empty>> {
        self.client().set_beam_config(req).await
    }

    async fn get_beam_enable(
        &self,
        req: Request<rms::SatBeamId>,
    ) -> Result<Response<rms::BeamEnableSetting>> {
        self.client().get_beam_enable(req).await
    }

    async fn set_beam_enable(
        &self,
        req: Request<rms::BeamEnableSetting>,
    ) -> Result<Response<Empty>> {
        self.client().set_beam_enable(req).await
    }

    async fn get_sat_summary(
        &self,
        req: Request<rms::GetSatSummaryRequest>,
    ) -> Result<Response<rms::SatSummary>> {
        self.client().get_sat_summary(req).await
    }

    async fn get_gsc_group(&self, req: Request<rms::GscGroup>) -> Result<Response<rms::GscGroup>> {
        self.client().get_gsc_group(req).await
    }

    async fn set_gsc_group(&self, req: Request<rms::GscGroup>) -> Result<Response<Empty>> {
        self.client().set_gsc_group(req).await
    }

    async fn del_gsc_group(&self, req: Request<rms::GscGroup>) -> Result<Response<Empty>> {
        self.client().del_gsc_group(req).await
    }

    async fn set_gsc_group_resource_limits(
        &self,
        req: Request<rms::GscGroupResourceLimits>,
    ) -> Result<Response<Empty>> {
        self.client().set_gsc_group_resource_limits(req).await
    }

    async fn get_gsc_group_resource_limits(
        &self,
        req: Request<rms::GetGscGroupResourceLimitsRequest>,
    ) -> Result<Response<rms::GscGroupResourceLimits>> {
        self.client().get_gsc_group_resource_limits(req).await
    }

    async fn get_beam_power(
        &self,
        req: Request<rms::GetBeamPowerRequest>,
    ) -> Result<Response<rms::BeamPower>> {
        self.client().get_beam_power(req).await
    }

    async fn set_beam_power(&self, req: Request<rms::BeamPower>) -> Result<Response<Empty>> {
        self.client().set_beam_power(req).await
    }

    async fn provision_beam(&self, req: Request<rms::InventorySet>) -> Result<Response<Empty>> {
        self.client().provision_beam(req).await
    }
}

pub struct GscApiProxy {
    endpoint: Endpoint,
}

impl GscApiProxy {
    pub async fn new(target: &str) -> Self {
        Self {
            endpoint: Endpoint::from_shared(target.to_string())
                .expect("Invalid endpoint URL in GscApiProxy::new"),
        }
    }

    // create a client to connect to the target server
    fn client(&self) -> GscGatewayApiClient<Channel> {
        let conn = self.endpoint.connect_lazy();
        GscGatewayApiClient::new(conn)
    }
}

#[tonic::async_trait]
impl rms::gsc_gateway_api_server::GscGatewayApi for GscApiProxy {
    async fn get_gsc_connections(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<rms::GscConnectionList>> {
        log::warn!("Proxying get_gsc_connections request to GSC endpoint");
        self.client().get_gsc_connections(request).await
    }

    /// GSC 快照周期设置
    async fn set_snapshot_period(
        &self,
        request: tonic::Request<rms::GscSnapshotPeriod>,
    ) -> Result<Response<Empty>> {
        self.client().set_snapshot_period(request).await
    }
}

#[cfg(test)]
mod tests {
    use crate::rms::rms_config_api_client::RmsConfigApiClient;
    use tinc::well_known::prost::Empty;
    use tonic::{Request, transport::Endpoint};

    #[tokio::test]
    async fn test_grpc_client() {
        let conn = Endpoint::from_static("https://127.0.0.1:10000/")
            .connect()
            .await
            .unwrap();
        let mut client = RmsConfigApiClient::new(conn);
        client
            .list_satellites(Request::new(Empty::default()))
            .await
            .unwrap();
    }
}
