use std::sync::Arc;

use crate::crawler_ticker_details_impl::Service;
use chrono::NaiveDate;
use crawler_mod_ticker_details_io::crawler_ticker_details as io;
use database_mod_ticker_details_fixture::ticker_details_fixture::ServiceFake as DBServiceFake;
use database_mod_ticker_details_io::ticker_details as database;
use io::Api;
use network_mod_ticker_details_fixture::ticker_details_fixture::ServiceFake as NetServiceFake;
use network_mod_ticker_details_io::ticker_details as network;
use policy_mod_fixture::policy_fixture::ServiceFake as PolicyServiceFake;
use policy_mod_io::policy;
use snp_mod_fixture::snp_fixture::ServiceFake as SnPServiceFake;
use snp_mod_io::snp;

#[tokio::test]
async fn call_crawl_ticker_details() -> io::Result<()> {
    let format = "%B %e, %Y";
    let service = Service::new();
    let data = io::Data {
        ticker: snp::SnP::AAPL,
        date: NaiveDate::parse_from_str("April 17, 2024", format).unwrap(),
    };
    let deps = given_deps();
    let mut policy_data = policy::Data::default();

    service
        .crawl_ticker_details(deps, &data, &mut policy_data)
        .await?;

    Ok(())
}

fn given_deps() -> Arc<dyn io::Depends + Send + Sync> {
    Arc::new(DependsFake {})
}

struct DependsFake {}

impl DependsFake {}

impl io::Depends for DependsFake {
    fn provide_snp(&self) -> Arc<dyn snp::Api + Send + Sync> {
        Arc::new(SnPServiceFake {})
    }

    fn provide_policy(&self) -> Arc<dyn policy::Api + Send + Sync> {
        Arc::new(PolicyServiceFake {})
    }

    fn provide_network_ticker_details(&self) -> Arc<dyn network::Api + Send + Sync> {
        Arc::new(NetServiceFake {})
    }

    fn provide_database_ticker_details(&self) -> Arc<dyn database::Api + Send + Sync> {
        Arc::new(DBServiceFake {})
    }
}
