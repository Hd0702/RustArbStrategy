use base64;
use hmac::{Hmac, Mac};
use sha2::{Sha256, Sha512, Digest};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::str;
use reqwest::{Client, Method, header::HeaderMap, header::HeaderValue, Error, ClientBuilder};
use std::collections::HashMap;
use std::fmt::format;
use base64::{Engine as Base64Engine, engine::{self, general_purpose}};
use once_cell::sync::Lazy;

const API_URL: &'static str = "https://api.kraken.com";

pub struct KrakenClient {
    pub api_key: String,
    pub api_secret: String,
}

static CLIENT : Lazy<Client> = Lazy::new(|| ClientBuilder::new().timeout(Duration::from_secs(10)).build().expect("Failed to build client"));
impl KrakenClient {
    pub fn get_price(&self) -> Result<String, Error> {
        self.get_balance()
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