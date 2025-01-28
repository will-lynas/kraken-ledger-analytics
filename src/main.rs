use std::{collections::HashMap, fs::File};

use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use csv::ReaderBuilder;
use serde::{de, Deserialize, Deserializer};

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
struct TxId(String);

#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize)]
struct RefId(String);

#[derive(Clone, PartialEq, Debug, Deserialize)]
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

#[derive(Clone, PartialEq, Debug, Deserialize)]
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
#[derive(Clone, Debug, Deserialize)]
struct Transaction {
    txid: TxId,
    refid: RefId,
    #[serde(deserialize_with = "deserialize_datetime")]
    time: DateTime<Utc>,
    r#type: TransactionType,
    // subtype: String,
    // aclass: String,
    asset: Asset,
    // wallet: String,
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

#[allow(dead_code)]
#[derive(Debug)]
struct Position {
    open: Transaction,
    close: Option<Transaction>,
    rollovers: Vec<Transaction>,
}

fn main() -> Result<()> {
    let file = File::open("input/ledgers.csv")?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let transactions: Vec<Transaction> = rdr.deserialize().map(|t| t.unwrap()).collect();

    let mut positions: HashMap<RefId, Position> = HashMap::new();

    for transaction in transactions {
        match transaction.r#type {
            TransactionType::Margin => {
                if let Some(position) = positions.get_mut(&transaction.refid) {
                    assert!(position.close.is_none());
                    position.close = Some(transaction.clone());
                } else {
                    positions.insert(
                        transaction.refid.clone(),
                        Position {
                            open: transaction.clone(),
                            close: None,
                            rollovers: Vec::new(),
                        },
                    );
                }
            }
            TransactionType::Rollover => {
                let position = positions.get_mut(&transaction.refid).unwrap();
                position.rollovers.push(transaction.clone());
            }
            _ => {}
        }
    }

    dbg!(positions);

    // println!("{margins:#?}");

    Ok(())
}
