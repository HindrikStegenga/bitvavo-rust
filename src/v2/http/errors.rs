use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Debug)]
pub enum HttpApiError {
    Reqwest(reqwest::Error),
    InvalidURL,
    Deserialize(serde_json::Error),
}

impl Display for HttpApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpApiError::Reqwest(err) => write!(f, "{}", err),
            HttpApiError::Deserialize(err) => write!(f, "{}", err),
            HttpApiError::InvalidURL => write!(f, "invalid url"),
        }
    }
}
impl Error for HttpApiError {}

impl From<reqwest::Error> for HttpApiError {
    fn from(e: reqwest::Error) -> Self {
        HttpApiError::Reqwest(e)
    }
}
impl From<serde_json::Error> for HttpApiError {
    fn from(e: serde_json::Error) -> Self {
        HttpApiError::Deserialize(e)
    }
}