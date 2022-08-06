use crate::config as cfg;

use std::collections::HashMap;
use std::error::Error;

use reqwest::header::{HeaderMap, AUTHORIZATION};

/// Returns the shortened URL.
///
/// From a long URL, it makes a shorter URL using the bit.ly API v4.
pub fn shorten(url: &str) -> Result<String, Box<dyn Error>> {
    let api_key = &*cfg::API_KEY;
    let api_url = cfg::API_URL;

    let shorten_url = format!("{api_url}/shorten");
    let params = HashMap::from([("long_url", url)]);
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {api_key}").parse().unwrap());
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(shorten_url)
        .json(&params)
        .headers(headers)
        .send()?;
    let body = response.text()?;
    let v: serde_json::Value = serde_json::from_str(&body)?;
    let result = v["id"].as_str();
    match result {
        Some(value) => Ok(value.to_string()),
        _ => Err("problem with the 'id' field")?,
    }
}

/// Expands a bit.ly shortened URL and returns the long URL.
pub fn expand(url_id: &str) -> Result<String, Box<dyn Error>> {
    let api_key = &*cfg::API_KEY;
    let api_url = cfg::API_URL;

    let expand_url = format!("{api_url}/expand");
    let params = HashMap::from([("bitlink_id", url_id)]);
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {api_key}").parse().unwrap());
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(expand_url)
        .json(&params)
        .headers(headers)
        .send()?;
    let body = response.text()?;
    let v: serde_json::Value = serde_json::from_str(&body)?;
    let result = v["long_url"].as_str();
    match result {
        Some(value) => Ok(value.to_string()),
        _ => Err("problem with the 'long_url' field")?,
    }
}
