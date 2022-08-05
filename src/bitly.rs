use crate::config as cfg;

use std::collections::HashMap;

use reqwest::header::{HeaderMap, AUTHORIZATION};

/// Returns the shortened URL.
///
/// From a long URL, it makes a shorter URL using the bit.ly API v4.
pub fn shorten(url: &str) -> String {
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
        .send()
        .expect("calling the bit.ly API failed");
    let body = response.text().expect("cannot extract response body");
    let v: serde_json::Value =
        serde_json::from_str(&body).expect("cannot convert response body to json");
    v["id"]
        .as_str()
        .expect("problem with the 'id' field")
        .to_string()
}

/// Expands a bit.ly shortened URL and returns the long URL.
pub fn expand(url_id: &str) -> String {
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
        .send()
        .expect("calling the bit.ly API failed");
    let body = response.text().expect("cannot extract response body");
    let v: serde_json::Value =
        serde_json::from_str(&body).expect("cannot convert response body to json");
    v["long_url"]
        .as_str()
        .expect("problem with the 'long_url' field")
        .to_string()
}
