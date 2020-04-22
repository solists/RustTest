//#![deny(warnings)]
#![warn(rust_2018_idioms)]

mod http_connector;
use http_connector::{Result, RequestManager};
mod common;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    
    let rq_mngr = RequestManager::new();

    rq_mngr.get_portfolio().await?;

    Ok(())
}
