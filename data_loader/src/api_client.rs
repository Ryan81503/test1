use serde_json::Value;
use ureq;

pub fn fetch_price_from_api(url: &str, json_path: &str) -> Result<f64, String> {
    let response: String = ureq::get(url)
        .call()
        .map_err(|e| e.to_string())?
        .into_string()
        .map_err(|e| e.to_string())?;

    let json: Value = serde_json::from_str(&response).map_err(|e| e.to_string())?;
    let price = json
        .pointer(json_path)
        .and_then(|v| v.as_f64())
        .ok_or_else(|| "Failed to extract price from JSON".to_string())?;

    Ok(price)
}