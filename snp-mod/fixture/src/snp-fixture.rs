use std::{collections::HashMap, sync::Arc};

use chrono::NaiveDate;
use snp_mod_io::snp as io;

pub struct ServiceFake;

impl io::Api for ServiceFake {
    fn get_snp(&self, _ticker: &str) -> io::Result<io::SnP> {
        Ok(io::SnP::AAPL)
    }

    fn get_symbol(&self, _snp: &io::SnP) -> Arc<str> {
        Arc::from("AAPL")
    }

    fn get_snp_map(&self) -> HashMap<io::SnP, io::Data> {
        HashMap::new()
    }

    fn get_snp_by_date(&self, _date: &NaiveDate) -> Vec<io::SnP> {
        vec![io::SnP::AAPL]
    }
}
