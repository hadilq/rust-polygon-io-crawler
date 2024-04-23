use std::sync::Arc;

use async_trait::async_trait;
use snp_mod_io::snp;

use crawler_mod_ticker_details_io::crawler_ticker_details as io;
use database_mod_ticker_details_io::ticker_details as database;
use network_mod_ticker_details_io::ticker_details as network;
use policy_mod_io::policy;

#[derive(Debug)]
pub struct Service;

/// The service to implement the API of Ticker_details.
impl Service {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl io::Api for Service {
    async fn crawl_ticker_details(
        &self,
        deps: Arc<dyn io::Depends + Send + Sync>,
        data: &io::Data,
        policy_data: &mut policy::Data,
    ) -> io::Result<()> {
        let policy = deps.provide_policy().clone();

        let snp = deps.provide_snp().clone();
        let network_service = deps.provide_network_ticker_details().clone();
        let database_service = deps.provide_database_ticker_details().clone();
        let network_depends = Arc::new(NetworkDepends { snp: snp.clone() });
        let database_depends = Arc::new(DatabaseDepends { snp: snp.clone() });

        let _ = match database_service
            .find_ticker_details(database_depends.clone(), &data.date, &data.ticker)
            .await
        {
            Ok(_) => return Ok(()),
            Err(e) => match e {
                database::Error::NotFound => (),
                _ => return Err(e)?,
            },
        };

        let network_data_response = network_service
            .get_ticker_details(network_depends.clone(), &data.ticker, &data.date)
            .await;

        let network_data = match network_data_response {
            Ok(data) => data,
            Err(_) => {
                policy.handle_request_rate(policy_data).await?;
                return Ok(());
            }
        };
        let database_data = database::Details {
            active: network_data.active,
            date: data.date,
            snp: data.ticker.clone(),
            share_class_shares_outstanding: network_data.share_class_shares_outstanding,
            total_employees: network_data.total_employees,
            weighted_shares_outstanding: network_data.weighted_shares_outstanding,
        };
        let _ = database_service
            .save_ticker_details(database_depends.clone(), &database_data)
            .await?;

        policy.handle_request_rate(policy_data).await?;
        Ok(())
    }
}

struct NetworkDepends {
    snp: Arc<dyn snp::Api + Send + Sync>,
}

impl network::Depends for NetworkDepends {
    fn provide_snp(&self) -> Arc<dyn snp::Api + Send + Sync> {
        return self.snp.clone();
    }
}

struct DatabaseDepends {
    snp: Arc<dyn snp::Api + Send + Sync>,
}

impl database::Depends for DatabaseDepends {
    fn provide_snp(&self) -> Arc<dyn snp::Api + Send + Sync> {
        return self.snp.clone();
    }
}
