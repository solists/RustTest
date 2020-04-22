use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};
use hyper::{Body, Method, Request};
use hyper::client::{HttpConnector};
use hyper_tls::HttpsConnector;
use std::fs;
use std::path::Path;

use crate::common::structs::{Urls, Config};

// A simple type alias so as to DRY.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct RequestManager {
    token: String,
    urls: Urls,
    //connector: HttpsConnector<HttpConnector>,
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

        RequestManager{token: config.token, urls: urls, client: Client::builder().build::<_, hyper::Body>(connector)}//connector: HttpsConnector::new(),}
    }

    async fn fetch_url(&self, url: &hyper::Uri) -> Result<()> {
        //let client = Client::builder().build::<_, hyper::Body>(self.connector);

        //let url = format!("{}{}", &self.urls.base_url_sandbox, &self.urls.snbx_register);
        /*let url = self.urls.base_url_sandbox.clone() + &self.urls.snbx_register;
        let url = url.parse::<hyper::Uri>().unwrap();
        if url.scheme_str() != Some("https") {
            println!("This example only works with 'https' URLs.");
            return Ok(());
        }*/

        let req = Request::builder()
            .method(Method::POST)
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

    pub async fn get_active_orders(&self) -> Result<()> {
        //let client = Client::builder().build::<_, hyper::Body>(self.connector);
        let url = self.urls.base_url_sandbox.clone() + &self.urls.snbx_register;
        let url = url.parse::<hyper::Uri>().unwrap();
        if url.scheme_str() != Some("https") {
            println!("This example only works with 'https' URLs.");
            return Ok(());
        }

        self.fetch_url(&url).await?;

        println!("\n\nDone! Get act orders.");

        Ok(())
    }
}