#![deny(warnings)]
#![warn(rust_2018_idioms)]
use hyper_tls::HttpsConnector;
use std::fs;
use serde::{Deserialize, Serialize};
use std::path::Path;

mod http_connector;
use http_connector::{Result, fetch_url};

#[derive(Serialize, Deserialize)]
struct Config {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct Urls {
    base_url : String,
    base_url_sandbox : String,
    snbx_register : String,
    snbx_set_cur_bal : String,
    snbx_set_pos_bal : String,
    snbx_remove : String,
    snbx_clear : String,
    get_active_orders : String,
    create_limit_order : String,
    create_market_order : String,
    cancel_order : String,
    get_client_portfolio : String,
    get_client_currencies : String,
    get_stocks_list : String,
    get_bonds_list : String,
    get_etf_list : String,
    get_currencies_list : String,
    get_candles_list : String,
    get_orderbook : String,
    get_by_figi : String,
    get_by_ticker : String,
    get_operations_list : String,
    get_user_accounts : String,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let path = Path::new("config/config.json");
    let contents = fs::read_to_string(path).expect("Error opening config.json");
    let config: Config = serde_json::from_str(&contents)?;
    let path = Path::new("config/urls.json");
    let contents =  fs::read_to_string(path).expect("Error opening urls.json");
    let urls: Urls =  serde_json::from_str(&contents)?;

    
    let url = urls.base_url_sandbox + &urls.snbx_register;
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("https") {
        println!("This example only works with 'https' URLs.");
        return Ok(());
    }
    let https = HttpsConnector::new();

    fetch_url(&url, https.clone(), &config.token).await?;
    fetch_url(&url, https.clone(), &config.token).await
}
