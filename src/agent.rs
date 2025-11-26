use std::error::Error;

use crate::proto::cogito_client::CogitoClient;

/// Connection with the Cogito agent.
///
/// Cloning this is cheap due to it just being a channel.
#[derive(Clone)]
pub struct CogitoAgent {
    client: CogitoClient<tonic::transport::Channel>,
}

impl CogitoAgent {
    pub async fn connect(url: String) -> Result<Self, Box<dyn Error>> {
        Ok(CogitoAgent {
            client: CogitoClient::connect(url).await?,
        })
    }
}
