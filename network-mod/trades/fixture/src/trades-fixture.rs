use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDate;
use network_mod_trades_io::trades as io;
use snp_mod_io::snp;

pub struct ServiceFake;

#[async_trait]
impl io::Api for ServiceFake {
    async fn get_trades(
        &self,
        _deps: Arc<dyn io::Depends + Send + Sync>,
        _ticker: &snp::SnP,
        _date: &NaiveDate,
    ) -> io::Result<()> {
        Ok(())
    }
}
