use std::fs::File;

use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use csv::ReaderBuilder;
use serde::{de, Deserialize, Deserializer};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Transaction {
    txid: String,
    refid: String,
    #[serde(deserialize_with = "deserialize_datetime")]
    time: DateTime<Utc>,
    r#type: String,
    subtype: String,
    aclass: String,
    asset: String,
    wallet: String,
    amount: f64,
    fee: f64,
    balance: f64,
}

fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
        .map(|dt| dt.and_utc())
        .map_err(de::Error::custom)
}

fn main() -> Result<()> {
    let file = File::open("input/ledgers.csv")?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    for result in rdr.deserialize() {
        let transaction: Transaction = result?;
        println!("{:?}", transaction);
    }

    Ok(())
}
