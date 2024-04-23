
use std::{sync::Arc, env, convert::Infallible, num::{TryFromIntError, ParseIntError}};

use chrono::{NaiveDate, Utc, Duration, Datelike};
use snp_mod_io::snp;
use snp_mod_impl::snp_impl::{self, WrapSnP};
use policy_mod_io::policy;
use policy_mod_impl::policy_impl;
use network_mod_ticker_details_io::ticker_details as network;
use network_mod_ticker_details_impl::ticker_details_impl as network_impl;
use database_mod_ticker_details_io::ticker_details as database;
use database_mod_ticker_details_impl::ticker_details_impl as database_impl;
use crawler_mod_ticker_details_io::crawler_ticker_details as crawler;
use crawler_mod_ticker_details_impl::crawler_ticker_details_impl as crawler_impl;
use crawler::Api;
use tokio_postgres::NoTls;

/// The main procedure to run the crawler.
#[tokio::main]
async fn main() -> Result<(), Error> {

    let service = crawler_impl::Service::new();
    let deps = provide_dependencies().await?;
    let snp = deps.provide_snp().clone();

    let format = "%B %e, %Y";
    let start_date = NaiveDate::parse_from_str("April 17, 2024", format).unwrap();
    let end_date = Utc::now().naive_utc().date() - Duration::try_days(1)
        .ok_or(Error::DurationDay("one day overflowed!".to_string()))?;
    let duration = end_date - start_date;
    let days: usize = duration.num_days().try_into()?;
    let mut policy_data = policy::Data::default();

    for date in end_date.iter_days().rev().take(days) {
        match date.weekday() {
            chrono::Weekday::Sat|
            chrono::Weekday::Sun => continue,
            _ => ()
        }

        for ticker in snp.get_snp_by_date(&date) {
            let crawler_data = crawler::Data { ticker, date, };
            println!(
                "ticker: {0}, date: {1}",
                &Into::<WrapSnP>::into(crawler_data.ticker.clone()),
                &crawler_data.date
            );
            service.crawl_ticker_details(deps.clone(), &crawler_data, &mut policy_data).await?;
        }
    }

    Ok(())
}

async fn provide_dependencies() -> Result<Arc<dyn crawler::Depends + Send + Sync>, Error> {
    let api_key = env::var("API_KEY").unwrap();
    let cool_down_time: u64 = env::var("POLICY_COOL_DOWN_TIME").unwrap().parse()?;
    let allowed_requests: usize = env::var("POLICY_ALLOWED_REQUESTS").unwrap().parse()?;
    let network_client = Arc::new(reqwest::Client::new());
    let database_client = Arc::new(init_database().await?);
    let snp = Arc::new(snp_impl::Service::new());
    let policy = Arc::new(policy_impl::Service::new(cool_down_time, allowed_requests));
    let network_ticker_details = Arc::new(network_impl::Service::new(
        network_client, api_key.into()
    )?);
    let database_ticker_details = Arc::new(database_impl::Service::new(database_client));
    Ok(Arc::new(CrawlerDepends {
        snp,
        policy,
        network_ticker_details,
        database_ticker_details,
    }))
}

struct CrawlerDepends {
    snp: Arc<dyn snp::Api + Send + Sync>,
    policy: Arc<dyn policy::Api + Send + Sync>,
    network_ticker_details: Arc<dyn network::Api + Send + Sync>,
    database_ticker_details: Arc<dyn database::Api + Send + Sync>,
}

impl CrawlerDepends {}

impl crawler::Depends for CrawlerDepends {
    fn provide_snp(&self) -> Arc<dyn snp::Api + Send + Sync> {
        self.snp.clone()
    }

    fn provide_policy(&self) -> Arc<dyn policy::Api + Send + Sync> {
        self.policy.clone()
    }

    fn provide_network_ticker_details(&self) -> Arc<dyn network::Api + Send + Sync> {
        self.network_ticker_details.clone()
    }

    fn provide_database_ticker_details(&self) -> Arc<dyn database::Api + Send + Sync> {
        self.database_ticker_details.clone()
    }
}

async fn init_database() -> database::Result<tokio_postgres::Client> {
    let db_user = env::var("DB_USER").unwrap();
    let db_password = env::var("DB_PASSWORD").unwrap();
    let db_port = env::var("DB_PORT").unwrap().parse::<u16>().unwrap();
    let db_name = env::var("DB_NAME").unwrap();

    let (client, connection) = tokio_postgres::Config::new()
        .user(&db_user)
        .password(&db_password)
        .host("localhost")
        .port(db_port)
        .dbname(&db_name)
        .connect(NoTls)
        .await?;

    // Spawn connection
    tokio::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("Connection error: {}", error);
        }
    });

    Ok(client)
}


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Db(#[from] database::Error),
    #[error("{0}")]
    Crawler(#[from] crawler::Error),
    #[error("{0}")]
    NetError(#[from] network::Error),
    #[error("{0}")]
    DurationDay(String),
    #[error("{0}")]
    DaysTooLong(#[from] TryFromIntError),
    #[error("{0}")]
    Infallible(#[from] Infallible),
    #[error("{0}")]
    ParseIntError(#[from] ParseIntError),
}

