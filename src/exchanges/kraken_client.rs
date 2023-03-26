use base64;
use hmac::{Hmac, Mac};
use sha2::{Sha256, Sha512, Digest};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::str;
use reqwest::{Client, Method, header::HeaderMap, header::HeaderValue, Error, ClientBuilder};
use std::collections::HashMap;
use std::fmt::format;
use std::str::FromStr;
use base64::{Engine as Base64Engine, engine::{self, general_purpose}};
use reqwest::header::HeaderName;
use once_cell::sync::Lazy;
use serde_json::{Value, Map };
use serde::{Serialize, Deserialize, Deserializer};
use crate::models::{asset, price};

const API_URL: &'static str = "https://api.kraken.com";

pub struct KrakenClient {
    pub api_key: String,
    pub api_secret: String
}

#[derive(Serialize, Deserialize)]
struct Depth {
    asks: Vec<Order>,
    bids: Vec<Order>
}

#[derive(Serialize, Deserialize)]
struct Order {
    #[serde(deserialize_with = "f64_from_str")]
    price: f64,
    #[serde(deserialize_with = "f64_from_str")]
    volume: f64,
    timestamp: f64
}

fn f64_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}


static CLIENT : Lazy<Client> = Lazy::new(|| ClientBuilder::new().timeout(Duration::from_secs(10)).build().expect("Failed to build client"));
impl KrakenClient {
    pub(crate) fn get_price(&self, pairs: Vec<&str>) -> Result<price::Price, Error> {
        let headers: HashMap<String, String> = HashMap::from([("pair".to_string(), pairs.join(","))]);
        let result: String = self.public_get("/0/public/Depth", headers)?;
        let parsed: Value = serde_json::from_str(&result).expect("Failed to parse json");
        let depth_result: Depth = serde_json::from_value(parsed["result"]["ETHUSDT"].clone()).unwrap();
        // Fix for multiple assets
        Ok(price::Price {
            asset: asset::Asset::ETHUSDT,
            price: depth_result.asks[0].price
        })
    }

    #[tokio::main]
    async fn public_get(&self, endpoint: &str, headers: HashMap<String, String>) -> Result<String, Error> {
        let url = format!("{}{}?{}", API_URL, endpoint, headers.into_iter().map(|k| format!("{}={}", k.0, k.1)).collect::<Vec<String>>().join("&"));
        let response = CLIENT.get(&url)
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }
    // currently basic example
    #[tokio::main]
    async fn get_balance(&self) -> Result<String, Error> {
        let endpoint = "/0/private/TradeBalance";
        let url = format!("{}{}", API_URL, endpoint);
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_micros().to_string();
        let post_data = format!("nonce={}", nonce);
        let sig = self.generate_signature(&endpoint, &nonce, &post_data).await;
        let response = CLIENT.post(&url)
            .header("API-Key", &self.api_key)
            .header("API-Sign", self.generate_signature(&endpoint, &nonce, &post_data).await)
            .form(&[("nonce", nonce)])
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }

    async fn generate_signature(&self, path: &str, nonce: &str, post_data: &str) -> String {
        let encoded = format!("{}{}", nonce, post_data);
        let mut message = path.as_bytes().to_vec();
        message.extend_from_slice(&Sha256::digest(encoded.as_bytes()).as_slice());
        type HmacSha512 = Hmac::<Sha512>;
        let mut secret_key = HmacSha512::new_from_slice(engine::general_purpose::STANDARD.decode(&self.api_secret.as_bytes()).unwrap().as_slice()).unwrap();
        secret_key.update(&message);
        let sigdigest = secret_key.finalize().into_bytes();
        let result = engine::general_purpose::STANDARD.encode(&sigdigest).to_string();
        result
    }
}