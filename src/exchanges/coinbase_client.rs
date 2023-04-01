use std::collections::HashMap;
use std::error::Error;
use hex;
use hmac::{Hmac, Mac};
use reqwest::ClientBuilder;
use sha2::{Sha256, Digest};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::str;
use crate::models::{asset, price};
use crate::utils::{ http::CLIENT, traits::LetTrait };
use serde_json::{Value, Map, json};
use serde::{Serialize, Deserialize, Deserializer};
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

const API_URL: &'static str = "https://api.coinbase.com";

pub struct CoinbaseClient {
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Serialize)]
struct OrderRequest {
    client_order_id: String,
    product_id: String,
    side: String,
    order_config: OrderConfig
}

#[derive(Serialize)]
struct OrderConfig {
    market_market_ioc: MarketOrder
}

#[derive(Serialize)]
struct MarketOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    quote_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    base_size: Option<String>
}

// assume conversion rate is 0% since the spread is baked into the get_price API :(
impl CoinbaseClient {

    pub fn get_price(&self) -> Result<price::Price, Box<dyn Error>> {
        let response: String = self.private(&"GET", &"/api/v3/brokerage/products/ETH-USDT", "".to_string())?;
        let serde_response = &serde_json::from_str::<Value>(&response).expect("Failed to parse json")["price"];
        match serde_response.as_str() {
            Some(price) => Ok(price::Price {
                price: price.parse()?,
                asset: asset::Asset::ETHUSDT
            }),
            None => Err("Failed to parse price".into())
        }
    }

    pub fn private_get(&self) {
        println!("private get {}", self.private("GET", "/api/v3/brokerage/products/BTC-USD/candles?start=1606039200&end=1607115600&granularity=ONE_HOUR", "".to_string()).unwrap());
    }

    pub fn buy(&self) -> Result<String, Box<dyn Error>> {
        let endpoint = "/api/v3/brokerage/orders";
        let method = "POST";
        // let request = OrderRequest {
        //     client_order_id:  Uuid::new_v4().to_string(),
        //     product_id: "ETH-USDT".to_string(),
        //     side: "BUY".to_string(),
        //     order_config: OrderConfig {
        //         market_market_ioc: MarketOrder {
        //             // Find a better way to determine quote and base size
        //             quote_size: Some("0.1".to_string()),
        //             base_size: None
        //         }
        //     }
        // };
        let request = json!({
            "client_order_id": Uuid::new_v4().to_string(),
            "product_id": "ETH-USDT",
            "side": "BUY",
            "order_config": {
                "market_market_ioc": {
                    "quote_size": "0.1",
                }
            }
        });
        self.private(method, endpoint, request.to_string())
    }

    #[tokio::main]
    async fn private(&self, method: &str, endpoint: &str, data: String) -> Result<String, Box<dyn Error>> {
        let url = format!("{}{}", API_URL, endpoint);
        let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs().to_string();
        let signature = self.generate_signature(&method.to_string(), &endpoint.to_string(), &data, &since_the_epoch).await;
        println!("Signature: {} and url: {} and since_the_epoch: {} and body {}", &signature, &url, &since_the_epoch, &data);
        let result = CLIENT.let_owned(|client| {
            if method == "post" { client.post(&url).body(data) } else { client.get(&url) }
        }).header("Content-Type", "application/json")
            .header("CB-ACCESS-KEY", &self.api_key)
            .header("CB-ACCESS-SIGN", signature)
            .header("CB-ACCESS-TIMESTAMP", since_the_epoch)
            .send()
            .await?
            .text()
            .await;
        Ok(result?)
    }

    async fn generate_signature(&self, method: &String, request_path: &String, body: &String, timestamp: &String) -> String {
        let path = request_path.split('?').collect::<Vec<&str>>()[0];
        let message = format!("{}{}{}{}", timestamp, method, path, body);
        println!("THINGS {}", message);
        let mut mac = HmacSha256::new_from_slice(&self.api_secret.as_bytes()).unwrap();
        mac.update(message.as_bytes());

        let code_bytes = mac.finalize().into_bytes();

        return hex::encode(code_bytes);
    }
}
