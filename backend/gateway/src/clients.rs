use crate::config::GatewayConfig;
use crate::proto::{auth, content, engagement, statistics};
use tonic::transport::{Channel, Endpoint};

#[derive(Clone)]
pub struct Clients {
    auth: Channel,
    content: Channel,
    engagement: Channel,
    statistics: Channel,
}

impl Clients {
    pub async fn new(config: &GatewayConfig) -> Result<Self, tonic::transport::Error> {
        let auth = Endpoint::from_shared(config.auth_grpc_addr.clone())?
            .connect()
            .await?;
        let content = Endpoint::from_shared(config.content_grpc_addr.clone())?
            .connect()
            .await?;
        let engagement = Endpoint::from_shared(config.engagement_grpc_addr.clone())?
            .connect()
            .await?;
        let statistics = Endpoint::from_shared(config.statistics_grpc_addr.clone())?
            .connect()
            .await?;

        Ok(Self {
            auth,
            content,
            engagement,
            statistics,
        })
    }

    pub fn auth_client(&self) -> auth::auth_service_client::AuthServiceClient<Channel> {
        auth::auth_service_client::AuthServiceClient::new(self.auth.clone())
    }

    pub fn content_client(&self) -> content::content_service_client::ContentServiceClient<Channel> {
        content::content_service_client::ContentServiceClient::new(self.content.clone())
    }

    pub fn engagement_client(
        &self,
    ) -> engagement::engagement_service_client::EngagementServiceClient<Channel> {
        engagement::engagement_service_client::EngagementServiceClient::new(self.engagement.clone())
    }

    pub fn statistics_client(
        &self,
    ) -> statistics::statistics_service_client::StatisticsServiceClient<Channel> {
        statistics::statistics_service_client::StatisticsServiceClient::new(self.statistics.clone())
    }
}
