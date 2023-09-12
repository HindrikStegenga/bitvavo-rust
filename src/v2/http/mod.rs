mod errors;
mod structures;
mod urls;

use errors::*;
use structures::*;
use urls::*;

pub struct BitvavoClient {
    client: reqwest::Client,
}

impl Default for BitvavoClient {
    fn default() -> Self {
        Self::new()
    }
}

impl BitvavoClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_time(&self) -> std::result::Result<u64, HttpApiError> {
        let body = self
            .client
            .get(ENDPOINT_GET_TIME_URL)
            .send()
            .await?
            .bytes()
            .await
            .unwrap();
        Ok(serde_json::from_slice::<Time>(&body)?.time)
    }

    pub async fn get_market(&self, market: &str) -> Result<Market, HttpApiError> {
        let url = reqwest::Url::parse_with_params(ENDPOINT_GET_MARKETS_URL, [("market", market)])
            .map_err(|_| HttpApiError::InvalidURL)?;
        let bytes = self.client.get(url).send().await?.bytes().await.unwrap();
        Ok(serde_json::from_slice::<Market>(&bytes)?)
    }

    pub async fn get_markets(&self) -> Result<Vec<Market>, HttpApiError> {
        let bytes = self
            .client
            .get(ENDPOINT_GET_MARKETS_URL)
            .send()
            .await?
            .bytes()
            .await
            .unwrap();
        Ok(serde_json::from_slice::<Vec<Market>>(&bytes)?)
    }

    pub async fn get_asset(&self, symbol: &str) -> Result<Asset, HttpApiError> {
        let url = reqwest::Url::parse_with_params(ENDPOINT_GET_ASSETS_URL, [("symbol", symbol)])
            .map_err(|_| HttpApiError::InvalidURL)?;
        let bytes = self.client.get(url).send().await?.bytes().await.unwrap();
        Ok(serde_json::from_slice::<Asset>(&bytes)?)
    }

    pub async fn get_assets(&self) -> Result<Vec<Asset>, HttpApiError> {
        let bytes = self
            .client
            .get(ENDPOINT_GET_ASSETS_URL)
            .send()
            .await?
            .bytes()
            .await
            .unwrap();
        Ok(serde_json::from_slice::<Vec<Asset>>(&bytes)?)
    }

    pub async fn get_orderbook_snapshot(
        &self,
        market: &str,
        depth: Option<u32>,
    ) -> Result<OrderbookSnapshot, HttpApiError> {
        let url = if let Some(depth) = depth {
            reqwest::Url::parse_with_params(
                &((ENDPOINT_BASE_URL.to_string() + market).to_string() + "/book"),
                [("depth", depth.to_string())],
            )
            .map_err(|_| HttpApiError::InvalidURL)?
        } else {
            reqwest::Url::parse(&((ENDPOINT_BASE_URL.to_string() + market).to_string() + "/book"))
                .map_err(|_| HttpApiError::InvalidURL)?
        };
        let bytes = self.client.get(url).send().await?.bytes().await.unwrap();
        Ok(serde_json::from_slice::<OrderbookSnapshot>(&bytes)?)
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;

    #[tokio::test]
    async fn test_get_time() {
        let client = BitvavoClient::new();
        client.get_time().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_assets() {
        let client = BitvavoClient::new();
        client.get_assets().await.unwrap();
        client.get_asset("BTC").await.unwrap();
    }

    #[tokio::test]
    async fn test_get_markets() {
        let client = BitvavoClient::new();
        client.get_markets().await.unwrap();
        client.get_market("BTC-EUR").await.unwrap();
    }

    #[tokio::test]
    async fn test_orderbook_snapshot() {
        let client = BitvavoClient::new();
        client
            .get_orderbook_snapshot("BTC-EUR", None)
            .await
            .unwrap();
    }
}
