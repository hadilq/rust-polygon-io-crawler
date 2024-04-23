use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDate;
use tokio_postgres::Client;

use database_mod_ticker_details_io::ticker_details as io;
use snp_mod_io::snp;

/// The service to implement the API of Ticker_details.
pub struct Service {
    client: Arc<Client>,
}

impl Service {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl io::Api for Service {
    async fn save_ticker_details(
        &self,
        deps: Arc<dyn io::Depends + Send + Sync>,
        details: &io::Details,
    ) -> io::Result<()> {
        let snp = deps.provide_snp();
        let snp_symbol_rc = snp.get_symbol(&details.snp).clone();
        let snp_symbol = &snp_symbol_rc[..];
        self.client
            .execute(
                "INSERT INTO ticker_details (
                active, date, snp, share_class_shares_outstanding,
                total_employees, weighted_shares_outstanding)
             VALUES ($1, $2, $3, $4, $5, $6)",
                &[
                    &details.active,
                    &details.date,
                    &snp_symbol,
                    &details.share_class_shares_outstanding,
                    &details.total_employees,
                    &details.weighted_shares_outstanding,
                ],
            )
            .await?;

        Ok(())
    }

    async fn find_ticker_details(
        &self,
        deps: Arc<dyn io::Depends + Send + Sync>,
        date: &NaiveDate,
        snp: &snp::SnP,
    ) -> io::Result<io::Details> {
        let snp_api = deps.provide_snp();
        let snp_symbol_rc = snp_api.get_symbol(snp).clone();
        let snp_symbol = &snp_symbol_rc[..];

        let list = self
            .client
            .query(
                "SELECT * FROM ticker_details WHERE date = $1 AND snp = $2 LIMIT 1",
                &[&date, &snp_symbol],
            )
            .await?;

        let result: tokio_postgres::Row = list.into_iter().nth(0).ok_or(io::Error::NotFound)?;
        Ok(io::Details {
            active: result.get::<usize, bool>(1),
            date: result.get::<usize, NaiveDate>(2),
            snp: snp.clone(),
            share_class_shares_outstanding: result.get::<usize, i64>(4),
            total_employees: result.get::<usize, i32>(5),
            weighted_shares_outstanding: result.get::<usize, i64>(6),
        })
    }
}
