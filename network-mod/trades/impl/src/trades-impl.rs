use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use chrono::NaiveDate;
use crossbeam::channel::{bounded, Receiver, Sender};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use snp_mod_io::snp::SnP;

use network_mod_trades_io::trades as io;

/// The result structure of resutl in Polygon.io API.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TradesResult {
    // A list of condition codes.
    conditions: Vec<i32>,
    // The trade correction indicator.
    correction: i32,
    // The exchange ID.
    exchange: i32,
    // The Trade ID which uniquely identifies a trade. These are unique per combination of ticker,
    // exchange, and TRF. For example: A trade for AAPL executed on NYSE and a trade for AAPL
    // executed on NASDAQ could potentially have the same Trade ID.
    id: Arc<str>,
    // The nanosecond accuracy Participant/Exchange Unix Timestamp. This is the timestamp of when
    // the trade was actually generated at the exchange.
    participant_timestamp: i64,
    // The price of the trade. This is the actual dollar value per whole share of this trade. A
    // trade of 100 shares with a price of $2.00 would be worth a total dollar value of
    // $200.00.
    price: f64,
    // The sequence number represents the sequence in which trade events happened. These are
    // increasing and unique per ticker symbol, but will not always be sequential (e.g., 1, 2, 6,
    // 9, 10, 11). Values reset after each trading session/day.
    sequence_number: i64,
    // The nanosecond accuracy SIP Unix Timestamp. This is the timestamp of when the SIP received
    // this trade from the exchange which produced it.
    sip_timestamp: i64,
    // The size of a trade (also known as volume).
    size: i32,
    // There are 3 tapes which define which exchange the ticker is listed on. These are integers in our objects which represent the letter of the alphabet. Eg: 1 = A, 2 = B, 3 = C.
    // Tape A is NYSE listed securities
    // Tape B is NYSE ARCA / NYSE American
    // Tape C is NASDAQ
    tape: i32,
    // The ID for the Trade Reporting Facility where the trade took place.
    trf_id: Option<i32>,
    // The nanosecond accuracy TRF (Trade Reporting Facility) Unix Timestamp. This is the timestamp
    // of when the trade reporting facility received this trade.
    trf_timestamp: Option<i64>,
}

/// The response structure of Polygon.io API.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TradesResponse {
    next_url: Option<Arc<str>>,
    request_id: Arc<str>,
    status: Arc<str>,
    results: Vec<TradesResult>,
}

/// The error structure of Polygon.io API.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TradesError {
    status: Arc<str>,
    request_id: Arc<str>,
    message: Arc<str>,
}

/// The service to implement the API of Trades.
#[derive(Debug)]
pub struct Service {
    url: Arc<str>,
    api_key: Arc<str>,
    client: Arc<Client>,
    sender: Sender<Vec<io::Data>>,
    pub channel: Receiver<Vec<io::Data>>,
}

impl Service {
    pub fn new(client: Arc<Client>, api_key: &str) -> io::Result<Self> {
        if api_key.is_empty() {
            return Err(io::Error::ApiKeyIsEmpty);
        }
        let (sender, channel) = bounded(0);

        Ok(Self {
            client,
            api_key: api_key.into(),
            url: "https://api.polygon.io/v3/trades/{TICKER}?sort=timestamp&order=asc".into(),
            sender,
            channel,
        })
    }
}

#[async_trait]
impl io::Api for Service {
    async fn get_trades(
        &self,
        deps: Arc<dyn io::Depends + Send + Sync>,
        ticker: &SnP,
        date: &NaiveDate,
    ) -> io::Result<()> {
        let formatted_date = date.format("%Y-%m-%d").to_string();

        let mut params = HashMap::new();

        params.insert("apiKey", self.api_key.as_ref());
        params.insert("timestamp", &formatted_date[..]);

        let snp = deps.provide_snp();
        let snp_symbol_rc = snp.get_symbol(ticker).clone();
        let snp_symbol = &snp_symbol_rc[..];
        dbg!(&snp_symbol);

        let client = &self.client;
        let sender = &self.sender;
        let mut url: String = self.url.to_string().replace("{TICKER}", snp_symbol);
        dbg!(&formatted_date);
        dbg!(&self.api_key);

        while !url.is_empty() {
            dbg!(&url);

            let response = client.get(&url).query(&params).send().await?;
            let status_code = response.status();
            dbg!(&status_code);

            let response_body = &response.text().await?;
            dbg!(&response_body);

            if status_code == StatusCode::OK {
                let trades_data: TradesResponse =
                    serde_json::from_str(response_body).map_err(io::Error::JsonParse)?;

                let result = trades_data
                    .results
                    .into_iter()
                    .map(|t| io::Data {
                        id: t.id,
                        price: t.price,
                        size: t.size,
                        participant_timestamp: t.participant_timestamp,
                    })
                    .collect();

                let _ = sender.send(result)?;
                url = if let Some(u) = trades_data.next_url {
                    u.to_string()
                } else {
                    "".to_string()
                };
            } else {
                let error_data: TradesError = serde_json::from_str(response_body)?;
                dbg!(&error_data);

                return Err(
                    io::Error::Server(url.to_string(), error_data.message.to_string()).into(),
                );
            }
        }
        Ok(())
    }
}
