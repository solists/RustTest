use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};
use hyper::{Body, Method, Request, Response};
use hyper::client::{HttpConnector};
use hyper_tls::HttpsConnector;
use std::fs;
use std::path::Path;
use hyper::http;

use crate::common::structs::{Urls, Config};

// A simple type alias so as to DRY.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub type HttpResult = std::result::Result<Response<hyper::body::Body>, hyper::error::Error>;

pub struct RequestManager {
    token: String,
    urls: Urls,
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

        RequestManager{token: config.token, urls: urls, client: Client::builder().build::<_, hyper::Body>(connector)}
    }

    async fn fetch_url(&self, url: &hyper::Uri, method: Method) -> Result<()> {
        let req = Request::builder()
            .method(method)
            .uri(url)
            .header("content-type", "application/json")
            .header("Authorization", &self.token)
            .body(Body::from(r#"{"library":"hyper"}"#))?;

        let mut res = self.client.request(req).await?;

        println!("Response: {}", res.status());
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
    
    
    async fn get_from_url(&self, url: &hyper::Uri) -> HttpResult {
        let req = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header("Authorization", &self.token)
            .body(Body::from(r#"{"library":"hyper"}"#))
            .expect("Error while constructing request");

        self.client.request(req).await
    }

    pub async fn get_currencies(&self) -> Result<()> {
        let url = self.urls.base_url.clone() + &self.urls.get_client_currencies;
        let url = url.parse::<hyper::Uri>().unwrap();

        self.get_from_url(&url).await?;

        println!("\n\nDone! Get currencies.");

        Ok(())
    }
    
    pub async fn get_portfolio(&self) -> Result<()> {
        let url = self.urls.base_url.clone() + &self.urls.get_client_portfolio;
        let url = url.parse::<hyper::Uri>().unwrap();

        let mut res = self.get_from_url(&url).await?;

        println!("Response: {}", res.status());
        println!("Headers: {:#?}\n", res.headers());
        println!("Body: {:#?}\n", res.body());

        // Stream the body, writing each chunk to stdout as we get it
        // (instead of buffering and printing at the end).
        while let Some(next) = res.data().await {
            let chunk = next?;
            io::stdout().write_all(&chunk).await?;
        }

        println!("\n\nDone get_from_url!");

        Ok(())
    }
}