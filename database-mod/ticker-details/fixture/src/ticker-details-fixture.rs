use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDate;
use database_mod_ticker_details_io::ticker_details as io;
use snp_mod_io::snp;

pub struct ServiceFake;

#[async_trait]
impl io::Api for ServiceFake {
    async fn save_ticker_details(
        &self,
        _deps: Arc<dyn io::Depends + Send + Sync>,
        _details: &io::Details,
    ) -> io::Result<()> {
        Ok(())
    }

    async fn find_ticker_details(
        &self,
        _deps: Arc<dyn io::Depends + Send + Sync>,
        _date: &NaiveDate,
        _snp: &snp::SnP,
    ) -> io::Result<io::Details> {
        let format = "%B %e, %Y";
        let date = NaiveDate::parse_from_str("April 17, 2024", format).unwrap();
        Ok(io::Details {
            active: true,
            date,
            snp: snp::SnP::AAPL,
            share_class_shares_outstanding: 33535433,
            total_employees: 4354,
            weighted_shares_outstanding: 533354,
        })
    }
}
