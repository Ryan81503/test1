mod bitcoin;
mod ethereum;
mod sp500;
mod pricing_trait;
mod api_client;

use bitcoin::Bitcoin;
use ethereum::Ethereum;
use sp500::SP500;
use pricing_trait::Pricing;
use std::thread;
use std::time::Duration;

fn main() {
    let assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin::new()),
        Box::new(Ethereum::new()),
        Box::new(SP500::new()),
    ];

    println!("Starting Financial Data Fetcher...");

    loop {
        for asset in &assets {
            match asset.fetch_price() {
                Ok(price) => {
                    println!("{} price: ${:.2}", asset.get_name(), price);
                    if let Err(err) = asset.save_price(price) {
                        eprintln!("Error saving {} price: {}", asset.get_name(), err);
                    }
                }
                Err(err) => {
                    eprintln!("Error fetching {} price: {}", asset.get_name(), err);
                }
            }
        }

        println!("Waiting for 10 seconds...");
        thread::sleep(Duration::from_secs(10));
    }
}