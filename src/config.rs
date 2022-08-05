use std::env;

use once_cell::sync::Lazy;

/// bit.ly API key
pub static API_KEY: Lazy<String> = Lazy::new(|| env::var("BITLY_ACCESS_TOKEN").unwrap_or_default());

/// base bit.ly API v4 URL
pub const API_URL: &str = "https://api-ssl.bit.ly/v4";
