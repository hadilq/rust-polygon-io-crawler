use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use chrono::NaiveDate;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use snp_mod_io::snp::SnP;

use network_mod_ticker_details_io::ticker_details as io;

/// The address structure in Polygon.io API.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TickerDetailsAddress {
    city: Arc<str>,
    postal_code: Arc<str>,
    state: Arc<str>,
}

/// The result structure of resutl in Polygon.io API.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TickerDetailsResult {
    // Whether or not the asset is actively traded. False means the asset has been delisted.
    active: bool,
    address: TickerDetailsAddress,
    // The CIK number for this ticker.
    cik: Arc<str>,
    // The name of the currency that this asset is traded with.
    currency_name: Arc<str>,
    // The last date that the asset was traded.
    delisted_utc: Option<Arc<str>>,
    // A description of the company and what they do/offer.
    description: Arc<str>,
    // The URL of the company's website homepage.
    homepage_url: Arc<str>,
    // The date that the symbol was first publicly listed in the format YYYY-MM-DD.
    list_date: Arc<str>,
    // The market type of the asset.
    market: Arc<str>,
    // The most recent close price of the ticker multiplied by weighted outstanding shares.
    market_cap: f64,
    // The name of the asset. For stocks/equities this will be the companies registered name. For crypto/fx this will be the name of the currency or coin pair.
    name: Arc<str>,
    // The phone number for the company behind this ticker.
    phone_number: Arc<str>,
    // The ISO code of the primary listing exchange for this asset.
    primary_exchange: Arc<str>,
    // Round lot size of this security.
    round_lot: i32,
    // The share Class OpenFIGI number for this ticker.
    share_class_figi: Arc<str>,
    // The recorded number of outstanding shares for this particular share class.
    share_class_shares_outstanding: i64,
    // The standard industrial classification code for this ticker.
    sic_code: Arc<str>,
    // A description of this ticker's SIC code.
    sic_description: Arc<str>,
    // The exchange symbol that this item is traded under.
    ticker: Arc<str>,
    // The root of a specified ticker. For example, the root of BRK.A is BRK.
    ticker_root: Arc<str>,
    // The suffix of a specified ticker. For example, the suffix of BRK.A is A.
    ticker_suffix: Option<Arc<str>>,
    // The approximate number of employees for the company.
    total_employees: i32,
    // The type of the asset.
    #[serde(rename = "type")]
    type_of_assset: Arc<str>,
    // The shares outstanding calculated assuming all shares of other share classes are converted
    // to this share class.
    weighted_shares_outstanding: i64,
}

/// The response structure of Polygon.io API.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TickerDetailsResponse {
    request_id: Arc<str>,
    status: Arc<str>,
    results: TickerDetailsResult,
}

/// The error structure of Polygon.io API.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TickerDetailsError {
    status: Arc<str>,
    request_id: Arc<str>,
    message: Arc<str>,
}

/// The service to implement the API of Ticker_details.
#[derive(Debug)]
pub struct Service {
    url: Arc<str>,
    api_key: Arc<str>,
    client: Arc<Client>,
}

impl Service {
    pub fn new(client: Arc<Client>, api_key: Arc<str>) -> io::Result<Self> {
        if api_key.is_empty() {
            return Err(io::Error::ApiKeyIsEmpty);
        }

        Ok(Self {
            client,
            api_key,
            url: "https://api.polygon.io/v3/reference/tickers/{TICKER}".into(),
        })
    }
}

#[async_trait]
impl io::Api for Service {
    async fn get_ticker_details(
        &self,
        deps: Arc<dyn io::Depends + Send + Sync>,
        ticker: &SnP,
        date: &NaiveDate,
    ) -> io::Result<io::Data> {
        let formatted_date = date.format("%Y-%m-%d").to_string();

        let mut params = HashMap::new();

        params.insert("apiKey", self.api_key.as_ref());
        params.insert("date", &formatted_date[..]);

        let snp = deps.provide_snp();
        let snp_symbol_rc = snp.get_symbol(ticker).clone();
        let snp_symbol = &snp_symbol_rc[..];

        let client = &self.client;
        let url = &self.url.to_string().replace("{TICKER}", snp_symbol);
        dbg!(&url);

        let response = client.get(url).query(&params).send().await?;

        let status_code = response.status();
        dbg!(&status_code);

        let response_body = &response.text().await?;
        dbg!(&response_body);

        if status_code == StatusCode::OK {
            let ticker_data: TickerDetailsResponse = serde_json::from_str(response_body)?;

            let result = ticker_data.results;

            Ok(io::Data {
                active: result.active,
                share_class_shares_outstanding: result.share_class_shares_outstanding,
                total_employees: result.total_employees,
                weighted_shares_outstanding: result.weighted_shares_outstanding,
            })
        } else {
            let error_data: TickerDetailsError = serde_json::from_str(response_body)?;

            Err(io::Error::Server(error_data.message.to_string()).into())
        }
    }
}
