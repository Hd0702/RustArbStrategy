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
use serde_json::{ Value, Map };
use serde::{Serialize, Deserialize, Deserializer};
use crate::models::{asset, price};
use crate::utils::traits::LetTrait;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderResult {
    txid: Vec<String>,
    descr: OrderDescription
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderDescription {
    order: String,
    close: Option<String>
}

fn f64_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}


static CLIENT : Lazy<Client> = Lazy::new(|| ClientBuilder::new().timeout(Duration::from_secs(10)).build().expect("Failed to build client"));

// assume conversion fee is 0.16% https://www.kraken.com/features/fee-schedule#spot-crypto
impl KrakenClient {
    pub(crate) fn get_price(&self, pairs: Vec<&str>) -> Result<price::Price, Error> {
        let headers: HashMap<String, String> = HashMap::from([("pair".to_string(), pairs.join(","))]);
        let result: String = self.public_get("/0/public/Depth", headers)?;
        let parsed: Value = serde_json::from_str(&result).expect("Failed to parse json");
        let depth_result: Depth = serde_json::from_value(parsed["result"]["ETHUSDT"].clone()).unwrap();
        // Fix for multiple assets
        Ok(price::Price {
            asset: asset::Asset::ETHUSDT,
            price: depth_result.bids[0].price
        })
    }

    // support multiple assets? also need a better way to determine price
    // volume is in ETH whether selling or buying
    pub fn buy(&self, price: Option<f64>) -> OrderResult {
        let mut json = serde_json::json!({
            "pair": "ETHUSDT",
            "type": "sell",
            "ordertype": "market",
            "volume": 0.01
        });
        if price.is_some() {
            let mut m = json.as_object_mut().unwrap();
            m.insert(String::from("price"), serde_json::json!(price.unwrap()));
        }
        match self.private("/0/private/AddOrder", json, "POST") {
            Ok(result) => {
                println!("BUY RESULT {}", &result);
                let parsed: Value = serde_json::from_str(&result).unwrap();

                serde_json::from_value::<OrderResult>(parsed["result"].clone()).unwrap()
            },
            Err(e) => panic!("Error: {}", e)
        }
    }

    // fee is in percent of 1% so 0.16% is 0.0016
    pub fn get_fee(&self) -> f64 {
        let mut json = serde_json::json!({
            "pair": "ETHUSDT"
        });
        match self.private("/0/private/TradeVolume", json, "POST") {
            Ok(result) => {
                println!("FEE RESULT {}", &result);
                let parsed: Value = serde_json::from_str(&result).unwrap();
                parsed["result"]["fees"]["ETHUSDT"]["fee"].as_str().expect("Failed to parse").parse::<f64>().unwrap()
            },
            Err(e) => panic!("Error: {}", e)
        }
    }

    #[tokio::main]
    async fn private(&self, endpoint: &str, mut form_fields: Value, method: &str) -> Result<String, Error> {
        let url = format!("{}{}", API_URL, endpoint);
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_micros().to_string();
        let mut payload = form_fields.clone();
        let m = payload.as_object_mut().unwrap();
        m.insert(String::from("nonce"), serde_json::json!(nonce));
        let payload = serde_json::to_value(m).unwrap();
        let sig = self.generate_signature(&endpoint, &nonce, &*serde_json::to_string(&payload).unwrap()).await;
        println!("payload {}", &payload);
        let response = CLIENT.let_owned(|client| {
            if method.eq("GET") { client.get(&url) } else { client.post(&url) }
        }).header("API-Key", &self.api_key)
            .header("API-Sign", sig)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&payload).unwrap())
            .send()
            .await?
            .text()
            .await?;
        // add better error handling
        Ok(response)
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