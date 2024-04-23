use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDate;
use serde::Deserialize;
use snp_mod_io::snp;

/// The data that is needed to interact with the Trades modules.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Data {
    pub id: Arc<str>,
    pub price: f64,
    pub size: i32,
    pub participant_timestamp: i64,
}

/// All possible errors of Trades modules.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to create an API client; can be invalid 'api_key'.")]
    ApiKeyIsEmpty,

    #[error("Failed to send a request to the service provider; can be invalid 'url', or 'api_key', or network IO problem: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Can't process the body text from the response: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Can't send the response over channel: {0}")]
    Channel(#[from] crossbeam::channel::SendError<Vec<Data>>),

    #[error("Provider server response error '{1}' for this url '{0}'.")]
    Server(String, String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// The API of Trades, which defines all the functionality of Trades modules.
#[async_trait]
pub trait Api {
    async fn get_trades(
        &self,
        deps: Arc<dyn Depends + Send + Sync>,
        ticker: &snp::SnP,
        date: &NaiveDate,
    ) -> Result<()>;
}

/// All the dependencies of the Trades modules to other modules.
pub trait Depends {
    fn provide_snp(&self) -> Arc<dyn snp::Api + Send + Sync>;
}

