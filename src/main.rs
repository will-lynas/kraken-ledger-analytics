use std::fs::File;

use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use csv::ReaderBuilder;
use serde::{de, Deserialize, Deserializer};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct TxId(String);

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct RefId(String);

#[derive(PartialEq, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TransactionType {
    Deposit,
    Trade,
    Margin,
    Rollover,
    Withdrawal,
    #[serde(other)]
    Other,
}

#[derive(PartialEq, Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum Asset {
    Usd,
    Gbp,
    Usdc,
    Btc,
    Sol,
    #[serde(other)]
    Other,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Transaction {
    txid: TxId,
    refid: RefId,
    #[serde(deserialize_with = "deserialize_datetime")]
    time: DateTime<Utc>,
    r#type: TransactionType,
    // subtype: String,
    // aclass: String,
    asset: Asset,
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

    let transactions: Vec<Transaction> = rdr.deserialize().map(|t| t.unwrap()).collect();
    dbg!(transactions);

    Ok(())
}
