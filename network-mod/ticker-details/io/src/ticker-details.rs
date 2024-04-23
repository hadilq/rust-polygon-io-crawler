use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDate;
use snp_mod_io::snp;

/// The data that is needed to interact with the Ticker-details modules.
#[derive(Debug)]
pub struct Data {
    pub active: bool,
    pub share_class_shares_outstanding: i64,
    pub total_employees: i32,
    pub weighted_shares_outstanding: i64,
}

/// All possible errors of Ticker-details modules.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to create an API client; can be invalid 'api_key'.")]
    ApiKeyIsEmpty,

    #[error("Failed to send a request to the service provider; can be invalid 'url', or 'api_key', or network IO problem: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Can't process the body text from the response: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Provider server response error '{0}'.")]
    Server(String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// The API of Ticker-details, which defines all the functionality of Ticker-details modules.
#[async_trait]
pub trait Api {
    async fn get_ticker_details(
        &self,
        deps: Arc<dyn Depends + Send + Sync>,
        ticker: &snp::SnP,
        date: &NaiveDate,
    ) -> Result<Data>;
}

/// All the dependencies of the Ticker-details modules to other modules.
pub trait Depends {
    fn provide_snp(&self) -> Arc<dyn snp::Api + Send + Sync>;
}

