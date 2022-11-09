use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub mod time;

#[derive(Debug)]
pub enum HttpApiError {
    Reqwest(reqwest::Error),
    Deserialize(serde_json::Error),
}

impl Display for HttpApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpApiError::Reqwest(err) => write!(f, "{}", err),
            HttpApiError::Deserialize(err) => write!(f, "{}", err),
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
