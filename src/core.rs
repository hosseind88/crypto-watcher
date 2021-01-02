use failure::{Fallible};
use reqwest::blocking::{Client};
use serde::{Deserialize};
use std::collections::HashMap;
use reqwest;
use url::Url;

use crate::utils::{clear_console, pretty_print};

#[derive(Deserialize, Debug)]
pub struct MarketData {
    pub current_price: HashMap<String, f32>,
    ath: HashMap<String, f32>,
    market_cap: HashMap<String, f32>,
    market_cap_rank: usize,
    high_24h: HashMap<String, f32>,
    low_24h: HashMap<String, f32>,
    price_change_24h: f32,
    price_change_percentage_24h_in_currency: HashMap<String, f32>,
}
#[derive(Deserialize, Debug)]
pub struct CoinData {
    id: String,
    symbol: String,
    name: String,
    image: HashMap<String, String>,
    market_cap_rank: usize,
    pub market_data: MarketData,
}

pub async fn get_data(url: &Url) -> Fallible<CoinData> {
    let resp: CoinData = Client::new().get(url.as_ref()).send().unwrap().json().unwrap();
    return Ok(resp);
}

pub async fn update_data(url: &Url) -> Fallible<()> {
    clear_console();
    let data = get_data(&url).await?;
    pretty_print(&data);
    return Ok(());
}
