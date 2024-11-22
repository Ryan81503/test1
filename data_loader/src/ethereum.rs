use serde::Deserialize;
use std::fs;
use crate::pricing_trait::Pricing;

pub struct Ethereum;

impl Ethereum {
    pub fn new() -> Self {
        Ethereum
    }
}

impl Pricing for Ethereum {
    fn get_name(&self) -> &str {
        "Ethereum"
    }

    fn fetch_price(&self) -> Result<f64, String> {
        let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd")
            .call()
            .map_err(|e| e.to_string())?;

        let json: EthereumResponse = serde_json::from_reader(response.into_reader())
            .map_err(|e| e.to_string())?;
        Ok(json.ethereum.usd)
    }

    fn save_price(&self, price: f64) -> Result<(), String> {
        let data = format!("Ethereum price: ${:.2}\n", price);
        fs::write("data/ethereum.txt", data).map_err(|e| e.to_string())
    }
}

#[derive(Deserialize)]
struct EthereumResponse {
    ethereum: Currency,
}

#[derive(Deserialize)]
struct Currency {
    usd: f64,
}