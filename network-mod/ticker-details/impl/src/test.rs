use std::env;
use std::sync::Arc;

use chrono::NaiveDate;
use snp_mod_fixture::snp_fixture::ServiceFake;
use snp_mod_io::snp;

use crate::ticker_details_impl::Service;
use io::Api;
use network_mod_ticker_details_io::ticker_details as io;

// It's not quite fit into the definiton of unit test, because it requests through network,
// but we need it to stay here for future fast check.
#[ignore]
#[tokio::test]
async fn call_aapl_details() -> io::Result<()> {
    let api_key = env::var("API_KEY").unwrap();
    let format = "%B %e, %Y";
    let client = reqwest::Client::new();
    let service = Service::new(Arc::new(client), api_key.into()).unwrap();
    let deps = given_deps();

    let response = service
        .get_ticker_details(
            deps,
            &snp::SnP::AAPL,
            &NaiveDate::parse_from_str("March 14, 2024", format).unwrap(),
        )
        .await;
    match response {
        Ok(r) => {
            assert_eq!(&15441880000, &r.share_class_shares_outstanding);
            assert_eq!(&15441881000, &r.weighted_shares_outstanding);
        }
        Err(e) => {
            dbg!(e);
        }
    }
    Ok(())
}

fn given_deps() -> Arc<dyn io::Depends + Send + Sync> {
    Arc::new(DependsFake {})
}

struct DependsFake {}

impl DependsFake {}

impl io::Depends for DependsFake {
    fn provide_snp(&self) -> Arc<dyn snp::Api + Send + Sync> {
        Arc::new(ServiceFake {})
    }
}
