pub trait Pricing {
    fn get_name(&self) -> &str;
    fn fetch_price(&self) -> Result<f64, String>;
    fn save_price(&self, price: f64) -> Result<(), String>;
    fn fetch_price_and_save(&self) -> Result<(), String> {
        let price = self.fetch_price()?;
        self.save_price(price)
    }
}