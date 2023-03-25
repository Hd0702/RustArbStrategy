use hex;
use hmac::{Hmac, Mac};
use reqwest::Error;
use reqwest::ClientBuilder;
use sha2::{Sha256, Digest};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::str;

type HmacSha256 = Hmac<Sha256>;

const API_URL: &'static str = "https://api.coinbase.com";

pub struct CoinbaseClient {
    pub api_key: String,
    pub api_secret: String,
}

impl CoinbaseClient {
    pub fn hello_world(&self) {
        self.list_accounts();
    }

    #[tokio::main]
    async fn list_accounts(&self) -> Result<(), Error> {
        let timeout = Duration::from_secs(10);
        let client = ClientBuilder::new().timeout(timeout).build()?;
        let method = "GET";
        let endpoint = "/api/v3/brokerage/accounts";
        let url = format!("{}{}", API_URL, endpoint);
        let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs().to_string();
        let signature = self.generate_signature(&method.to_string(), &endpoint.to_string(), &"".to_string(), &since_the_epoch).await;
        let response = client.get(&url)
            .header("accept", "application/json")
            .header("CB-ACCESS-KEY", &self.api_key)
            .header("CB-ACCESS-SIGN", signature)
            .header("CB-ACCESS-TIMESTAMP", since_the_epoch)
            .send()
            .await?
            .text()
            .await?;
        println!("Response: {}", response);
        Ok(())
    }

    async fn generate_signature(&self, method: &String, request_path: &String, body: &String, timestamp: &String) -> String {
        let path = request_path.split('?').collect::<Vec<&str>>()[0];
        let message = format!("{}{}{}{}", timestamp, method, path, body);
        let mut mac = HmacSha256::new_from_slice(&self.api_secret.as_bytes()).unwrap();
        mac.update(message.as_bytes());

        let code_bytes = mac.finalize().into_bytes();

        return hex::encode(&code_bytes.to_vec());
    }
}