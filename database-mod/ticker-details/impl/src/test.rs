use chrono::NaiveDate;
use std::env;
use std::sync::Arc;
use tokio_postgres::{Client, NoTls};

use crate::ticker_details_impl::Service;
use database_mod_ticker_details_io::ticker_details as io;
use io::Api;
use snp_mod_fixture::snp_fixture::ServiceFake;
use snp_mod_io::snp;

#[tokio::test]
async fn save_to_database() -> std::result::Result<(), io::Error> {
    let format = "%B %e, %Y";
    let db = Arc::new(init().await?);
    let db_ref = db.clone();
    let service = Service::new(db);
    let deps = given_deps();

    let details = io::Details {
        active: true,
        date: NaiveDate::parse_from_str("April 12, 2024", format).unwrap(),
        snp: snp::SnP::AAPL,
        share_class_shares_outstanding: 47539,
        total_employees: 3343,
        weighted_shares_outstanding: 0885503,
    };

    let _ = service.save_ticker_details(deps, &details).await?;

    let list = db_ref.query("SELECT * FROM ticker_details", &[]).await?;
    let result = list.last().unwrap();

    assert_eq!(result.get::<usize, bool>(1), details.active);
    assert_eq!(result.get::<usize, NaiveDate>(2), details.date);
    assert_eq!(
        result.get::<usize, i64>(4),
        details.share_class_shares_outstanding
    );
    assert_eq!(result.get::<usize, i32>(5), details.total_employees);
    assert_eq!(
        result.get::<usize, i64>(6),
        details.weighted_shares_outstanding
    );

    Ok(())
}

#[tokio::test]
async fn save_to_database_and_find() -> std::result::Result<(), io::Error> {
    let format = "%B %e, %Y";
    let db = Arc::new(init().await?);
    let service = Service::new(db);
    let deps = given_deps();

    let details = io::Details {
        active: true,
        date: NaiveDate::parse_from_str("April 12, 2024", format).unwrap(),
        snp: snp::SnP::AAPL,
        share_class_shares_outstanding: 47539,
        total_employees: 3343,
        weighted_shares_outstanding: 0885503,
    };

    let _ = service.save_ticker_details(deps.clone(), &details).await?;
    let new_detils = service
        .find_ticker_details(deps.clone(), &details.date, &details.snp)
        .await?;

    assert_eq!(new_detils.active, details.active);
    assert_eq!(new_detils.date, details.date);
    assert_eq!(new_detils.snp, details.snp);
    assert_eq!(
        new_detils.share_class_shares_outstanding,
        details.share_class_shares_outstanding
    );
    assert_eq!(new_detils.total_employees, details.total_employees);
    assert_eq!(
        new_detils.weighted_shares_outstanding,
        details.weighted_shares_outstanding
    );

    Ok(())
}

async fn init() -> io::Result<Client> {
    let db_user = env::var("DB_USER").unwrap();
    let db_password = env::var("DB_PASSWORD").unwrap();
    let db_port = env::var("DB_PORT").unwrap().parse::<u16>().unwrap();
    let db_name = env::var("DB_TEST_NAME").unwrap();

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
