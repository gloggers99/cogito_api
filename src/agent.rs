use std::{
    error::Error,
    ops::{Deref, DerefMut},
};

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

    pub fn get_client(&self) -> CogitoClient<tonic::transport::Channel> {
        // It is cheap to clone here.
        self.client.clone()
    }
}
