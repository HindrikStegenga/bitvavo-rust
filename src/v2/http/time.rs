use super::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Time {
    pub time: u64,
}

pub const ENDPOINT_GET_TIME_URL: &'static str = "https://api.bitvavo.com/v2/time";

pub async fn get_time() -> std::result::Result<Time, HttpApiError> {
    let body = reqwest::get(ENDPOINT_GET_TIME_URL)
        .await?
        .bytes()
        .await
        .unwrap();
    Ok(serde_json::from_slice::<Time>(&body)?)
}

#[cfg(test)]
mod test {
    use super::*;
    use reqwest::*;

    #[tokio::test]
    async fn test_time() {
        get_time().await.unwrap();
    }
}
