use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};
use hyper::{Body, Method, Request};
use hyper::client::{HttpConnector};
use hyper_tls::HttpsConnector;
use std::fs;
use std::path::Path;
use bytes::Bytes;

use crate::common::structs::{Urls, Config, ResponseKind};
use crate::data_storage::storage;
use crate::common::types::{Result, HttpResult};


pub struct RequestManager {
    token: String,
    urls: Urls,
    storage: &'static storage::Storage,
    client: Client<HttpsConnector<HttpConnector>, Body>,
}

impl RequestManager {
    // Warning IO operations
    pub fn new() -> RequestManager {
        let path = Path::new("config/config.json");
        let contents = fs::read_to_string(path).expect("Error opening config.json");
        let config: Config = serde_json::from_str(&contents).expect("Error while parsing config.json");
        let path = Path::new("config/urls.json");
        let contents =  fs::read_to_string(path).expect("Error opening urls.json");
        let urls: Urls =  serde_json::from_str(&contents).expect("Error while parsing urls.json");

        let connector = HttpsConnector::new();

        RequestManager{token: config.token, urls: urls,
            storage: unsafe {storage::STORAGES.take_storage() },
            client: Client::builder().build::<_, hyper::Body>(connector)}
    }

    async fn fetch_url(&self, url: &hyper::Uri, method: Method) -> Result<()> {
        let req = Request::builder()
            .method(method)
            .uri(url)
            .header("content-type", "application/json")
            .header("Authorization", &self.token)
            .body(Body::from(r#"{"library":"hyper"}"#))?;

        let mut res = self.client.request(req).await?;

        println!("Headers: {:#?}\n", res.headers());

        // Stream the body, writing each chunk to stdout as we get it
        // (instead of buffering and printing at the end).
        while let Some(next) = res.data().await {
            let chunk = next?;
            io::stdout().write_all(&chunk).await?;
        }

        println!("\n\nDone!");

        Ok(())
    }
    
    // Writes response body from get request to storage
    async fn get_from_url_to_storage(&self, url: &hyper::Uri) -> Result<()> {
        let req = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header("Authorization", &self.token)
            .body(Body::from(r#"{"library":"hyper"}"#))
            .expect("Error while constructing request");

        let mut res = self.client.request(req).await?;

        if !res.status().is_success() {
            println!("Error occured. \n{}", res.status());
            return Ok(())
        }
        let mut response_data: std::vec::Vec<Bytes> = Vec::new();

        // Stream the body, moving each chunk to vector as we get it
        while let Some(next) = res.data().await {
            response_data.push(next?);
        }

        self.storage.to_storage(response_data, ResponseKind::Portfolio).await
    }

    pub async fn get_currencies(&self) -> Result<()> {
        let url = self.urls.base_url.clone() + &self.urls.get_client_currencies;
        let url = url.parse::<hyper::Uri>().unwrap();

        self.get_from_url_to_storage(&url).await
    }
    
    pub async fn get_portfolio(&self) -> Result<()> {
        let url = self.urls.base_url.clone() + &self.urls.get_client_portfolio;
        let url = url.parse::<hyper::Uri>().unwrap();

        self.get_from_url_to_storage(&url).await
    }
}