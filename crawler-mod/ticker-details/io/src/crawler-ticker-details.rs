use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDate;
use database_mod_ticker_details_io::ticker_details as database;
use network_mod_ticker_details_io::ticker_details as network;
use policy_mod_io::policy;
use snp_mod_io::snp;

/// The data that is needed to interact with the Ticker-details modules.
#[derive(Debug)]
pub struct Data {
    pub ticker: snp::SnP,
    pub date: NaiveDate,
}

/// All possible errors of Ticker-details modules.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Db(#[from] database::Error),
    #[error("{0}")]
    Net(#[from] network::Error),
    #[error("{0}")]
    Policy(#[from] policy::Error),
    #[error("{0}")]
    SnP(#[from] snp::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// The API of Ticker-details, which defines all the functionality of Ticker-details modules.
#[async_trait]
pub trait Api {
    async fn crawl_ticker_details(
        &self,
        deps: Arc<dyn Depends + Send + Sync>,
        data: &Data,
        policy_data: &mut policy::Data,
    ) -> Result<()>;
}

/// All the dependencies of the Ticker-details modules to other modules.
pub trait Depends {
    fn provide_snp(&self) -> Arc<dyn snp::Api + Send + Sync>;
    fn provide_policy(&self) -> Arc<dyn policy::Api + Send + Sync>;
    fn provide_network_ticker_details(&self) -> Arc<dyn network::Api + Send + Sync>;
    fn provide_database_ticker_details(&self) -> Arc<dyn database::Api + Send + Sync>;
}

