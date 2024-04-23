use crate::snp_impl::{get_snp, get_snp_map};
use crate::snp_update::{format_all_updates, get_all_updates, get_all_updates_unformatted};
use chrono::NaiveDate;
use snp_mod_io::snp::SnP;

use crate::snp_by_date::get_snp_by_date;
use rstest::rstest;

#[test]
fn call_get_snp() {
    assert_eq!(SnP::AAPL, get_snp("AAPL").unwrap());
}

#[test]
fn call_snp_map() {
    let _ = get_snp_map();
}

#[test]
fn call_all_updates_unformatted() {
    let _ = get_all_updates_unformatted();
}

#[test]
fn call_format_all_updates() {
    match format_all_updates(get_all_updates_unformatted()) {
        Ok(_updates) => (),
        Err(e) => {
            dbg!(&e);
            panic!("error {}", e);
        }
    }
}

#[test]
fn call_all_updates() {
    let _ = get_all_updates();
}

#[test]
fn call_get_snp_by_date() {
    let format = "%B %e, %Y";
    let _ = get_snp_by_date(&NaiveDate::parse_from_str("December 7, 1999", format).unwrap());
}

#[rstest]
#[case("December 7, 1999", 513)]
#[case("June 17, 1997", 513)]
#[case("March 19, 2018", 505)]
fn call_count_get_snp_by_date(#[case] date: &str, #[case] count: usize) {
    let format = "%B %e, %Y";
    let list = get_snp_by_date(&NaiveDate::parse_from_str(date, format).unwrap());
    assert_eq!(count, list.len())
}
