use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDate;
use network_mod_ticker_details_io::ticker_details as io;
use snp_mod_io::snp;

pub struct ServiceFake;

#[async_trait]
impl io::Api for ServiceFake {
    async fn get_ticker_details(
        &self,
        _deps: Arc<dyn io::Depends + Send + Sync>,
        _ticker: &snp::SnP,
        _date: &NaiveDate,
    ) -> io::Result<io::Data> {
        Ok(io::Data {
            active: true,
            share_class_shares_outstanding: 453,
            total_employees: 5345,
            weighted_shares_outstanding: 7465,
        })
    }
}
