use std::collections::BTreeSet;

use chrono::NaiveDate;

use crate::{
    snp_impl::SnPList,
    snp_update::{get_all_updates, SnpTicker},
};
use snp_mod_io::snp::SnP;

pub fn get_snp_by_date(date: &NaiveDate) -> Vec<SnP> {
    let set = BTreeSet::from_iter(SnPList::VALUES2024.into_iter());
    let snps = get_all_updates()
        .into_iter()
        .filter(|update| update.date > *date)
        .fold(set, |mut acc, update| {
            if let SnpTicker::Available(t) = update.removed_ticker {
                acc.insert(t);
            }
            if let SnpTicker::Available(t) = update.added_ticker {
                if acc.contains(&t) == true {
                    acc.remove(&t);
                }
            }
            acc
        })
        .into_iter()
        .collect();
    snps
}
