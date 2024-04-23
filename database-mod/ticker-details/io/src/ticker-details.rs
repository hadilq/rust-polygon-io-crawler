use serde::{Deserialize, Serialize};
use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDate;
use snp_mod_io::snp;

/// The data that is needed to interact with the Ticker-details modules.
#[derive(Debug, Serialize, Deserialize)]
pub struct Details {
    pub active: bool,
    pub date: NaiveDate,
    pub snp: snp::SnP,
    pub share_class_shares_outstanding: i64,
    pub total_employees: i32,
    pub weighted_shares_outstanding: i64,
}

/// All possible errors of Ticker-details modules.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error with an embedded storage engine
    #[error("{0}")]
    Db(#[from] tokio_postgres::Error),
    #[error("Not found!")]
    NotFound,
}

pub type Result<T> = std::result::Result<T, Error>;

/// The API of Ticker-details, which defines all the functionality of Ticker-details modules.
#[async_trait]
pub trait Api {
    async fn save_ticker_details(
        &self,
        deps: Arc<dyn Depends + Send + Sync>,
        details: &Details,
    ) -> Result<()>;

    async fn find_ticker_details(
        &self,
        deps: Arc<dyn Depends + Send + Sync>,
        date: &NaiveDate,
        snp: &snp::SnP,
    ) -> Result<Details>;
}

/// All the dependencies of the Ticker-details modules to other modules.
pub trait Depends {
    fn provide_snp(&self) -> Arc<dyn snp::Api + Send + Sync>;
}

