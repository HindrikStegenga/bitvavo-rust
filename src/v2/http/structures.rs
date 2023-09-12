use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    pub time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DepositOrWithdrawalStatus {
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "MAINTENANCE")]
    Maintenance,
    #[serde(rename = "DELISTED")]
    Delisted,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub symbol: String,
    pub name: String,
    pub decimals: i32,
    #[serde(rename = "depositFee")]
    pub deposit_fee: String,
    #[serde(rename = "depositConfirmations")]
    pub deposit_confirmations: i32,
    #[serde(rename = "depositStatus")]
    pub deposit_status: DepositOrWithdrawalStatus,
    #[serde(rename = "withdrawalFee")]
    pub withdrawal_fee: String,
    #[serde(rename = "withdrawalMinAmount")]
    pub withdrawal_min_amount: String,
    #[serde(rename = "withdrawalStatus")]
    pub withdrawal_status: DepositOrWithdrawalStatus,
    pub networks: Vec<String>,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MarketStatus {
    #[serde(rename = "trading")]
    Trading,
    #[serde(rename = "halted")]
    Halted,
    #[serde(rename = "auction")]
    Auction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Market {
    pub market: String,
    pub status: MarketStatus,
    pub base: String,
    pub quote: String,
    #[serde(rename = "pricePrecision")]
    pub price_precision: u32,
    #[serde(rename = "minOrderInQuoteAsset")]
    pub min_order_in_quote_asset: String,
    #[serde(rename = "minOrderInBaseAsset")]
    pub min_order_in_base_asset: String,
    #[serde(rename = "orderTypes")]
    pub order_types: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderbookSnapshot {
    pub market: String,
    pub nonce: u64,
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
}
