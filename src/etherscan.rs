use std::collections::HashMap;
use std::error::Error;
use std::iter::Map;
use std::ops::Deref;

use reqwest::get;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use crate::models::networks::Network;
use std::sync::Arc;

pub struct Etherscan {
    pub api_key: Arc<String>,
    pub network: String,
    client: reqwest::Client,
}

impl Etherscan {
    pub fn new(api_key: String, network: Network) -> Etherscan {
        Etherscan {
            api_key: Arc::new(api_key),
            network: network.to_url(),
            client: reqwest::Client::builder().build().unwrap(),
        }
    }

    pub fn network(&mut self, network: Network) -> &mut Etherscan {
        self.network = network.to_url();
        self
    }

    pub fn api_key(&mut self, api_key: String) -> &mut Etherscan {
        self.api_key = Arc::new(api_key);
        self
    }

    pub async fn execute<T: DeserializeOwned>(&self, query: &Vec<(&str, &str)>) -> Result<T, reqwest::Error> {
        let mut query = query.clone();
        query.push(("apikey", self.api_key.as_str()));

        self.client
            .get(format!("{}", self.network))
            .query(&query)
            .send()
            .await?
            .json::<T>()
            .await
    }

    pub async fn execute_action<T: DeserializeOwned>(
        &self,
        module: &str,
        action: &str,
        query: &Vec<(&str, &str)>
    ) -> Result<T, reqwest::Error> {
        let mut query = query.clone();
        query.push(("module", module));
        query.push(("action", action));
        query.push(("apikey", self.api_key.as_str()));

        self.client
            .get(format!("{}", self.network))
            .query(&query)
            .send()
            .await?
            .json::<T>()
            .await
    }
}