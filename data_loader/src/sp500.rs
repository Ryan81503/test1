use reqwest::blocking::get;
use crate::pricing_trait::Pricing;
use serde::Deserialize;
use std::thread;
use std::time::Duration;

#[derive(Deserialize)]
struct YahooFinanceResponse {
    chart: Chart,
}

#[derive(Deserialize)]
struct Chart {
    result: Vec<ResultData>,
}

#[derive(Deserialize)]
struct ResultData {
    indicators: Indicators,
}

#[derive(Deserialize)]
struct Indicators {
    quote: Vec<Quote>,
}

#[derive(Deserialize)]
struct Quote {
    close: Vec<f64>,
}

pub struct SP500 {
    // Add any necessary fields here
}

impl SP500 {
    pub fn new() -> Self {
        SP500 {}
    }

    pub fn fetch_price(&self) -> Result<f64, String> {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d";

        let response = match get(url) {
            Ok(resp) => resp,
            Err(_) => return Err("Failed to fetch S&P 500 data".to_string()),
        };

        let body = match response.text() {
            Ok(body) => body,
            Err(_) => return Err("Failed to read response body".to_string()),
        };
        println!("Raw response: {}", body);

        if body.contains("Too Many Requests") {
            println!("API rate limit exceeded. Waiting for 10 seconds...");
            thread::sleep(Duration::from_secs(10)); 
            return self.fetch_price(); 
        }

        let response: Result<YahooFinanceResponse, _> = serde_json::from_str(&body);

        match response {
            Ok(parsed_response) => {
                if let Some(result) = parsed_response.chart.result.get(0) {
                    if let Some(close_prices) = result.indicators.quote.get(0) {
                        if let Some(&latest_price) = close_prices.close.last() {
                            return Ok(latest_price);
                        }
                    }
                }
                Err("Could not parse price".to_string())
            }
            Err(_) => Err("Failed to parse JSON response".to_string()),
        }
    }

    pub fn save_price(&self, price: f64) -> Result<(), String> {
        println!("Saving price: {}", price);
        Ok(())
    }
}

impl Pricing for SP500 {
    fn get_name(&self) -> &str {
        "S&P 500"
    }

    fn fetch_price(&self) -> Result<f64, String> {
        self.fetch_price()
    }

    fn save_price(&self, price: f64) -> Result<(), String> {
        self.save_price(price)
    }
}

fn main() {
    let sp500 = SP500::new();

    match sp500.fetch_price() {
        Ok(price) => {
            println!("Current S&P 500 Price: ${:.2}", price);
            if let Err(e) = sp500.save_price(price) {
                eprintln!("Error saving S&P 500 price: {}", e);
            }
        }
        Err(e) => eprintln!("Error fetching S&P 500 price: {}", e),
    }
}