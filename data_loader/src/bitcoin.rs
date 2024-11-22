use crate::pricing_trait::Pricing;
use serde::Deserialize;
use std::fs;

pub struct Bitcoin;

impl Bitcoin {
    pub fn new() -> Self {
        Bitcoin
    }
}

impl Pricing for Bitcoin {
    fn get_name(&self) -> &str {
        "Bitcoin"
    }

    fn fetch_price(&self) -> Result<f64, String> {
        let response = ureq::get("https://api.coindesk.com/v1/bpi/currentprice/BTC.json")
            .call()
            .map_err(|e| e.to_string())?;

        let json: BitcoinResponse = serde_json::from_reader(response.into_reader())
            .map_err(|e| e.to_string())?;
        Ok(json.bpi.USD.rate_float)
    }

    fn save_price(&self, price: f64) -> Result<(), String> {
        let data = format!("Bitcoin price: ${:.2}\n", price);
        fs::write("data/bitcoin.txt", data).map_err(|e| e.to_string())
    }
}

#[derive(Deserialize)]
struct BitcoinResponse {
    bpi: Bpi,
}

#[derive(Deserialize)]
struct Bpi {
    USD: Currency,
}

#[derive(Deserialize)]
struct Currency {
    rate_float: f64,
}