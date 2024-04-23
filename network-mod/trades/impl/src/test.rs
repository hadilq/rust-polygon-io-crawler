use std::env;
use std::sync::Arc;

use chrono::NaiveDate;
use snp_mod_fixture::snp_fixture::ServiceFake;
use snp_mod_io::snp;

use crate::trades_impl::Service;
use io::Api;
use network_mod_trades_io::trades as io;

// It's not quite fit into the definiton of unit test, because it requests through network,
// but we need it to stay here for future fast check.
#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn call_aapl_trades() -> io::Result<()> {
    let api_key = env::var("API_KEY").unwrap();
    let format = "%B %e, %Y";
    let client = reqwest::Client::new();
    let service = Service::new(Arc::new(client), &api_key).unwrap();
    let channel = service.channel.clone();
    let deps = given_deps();

    let _ = tokio::spawn(async move {
        let mut counter = 0;
        while counter < 2 {
            let response = channel.recv();
            match response {
                Ok(r) => {
                    dbg!(&r);
                }
                Err(e) => {
                    dbg!(e);
                    break;
                }
            }
            counter += 1;
        }
    });

    let _ = service
        .get_trades(
            deps,
            &snp::SnP::AAPL,
            &NaiveDate::parse_from_str("March 14, 2024", format).unwrap(),
        )
        .await;

    Ok(())
}

fn given_deps() -> Arc<dyn io::Depends + Send + Sync> {
    Arc::new(DependsFake {})
}

struct DependsFake {}

impl io::Depends for DependsFake {
    fn provide_snp(&self) -> Arc<dyn snp::Api + Send + Sync> {
        Arc::new(ServiceFake {})
    }
}
