#![deny(warnings)]
//#![warn(rust_2018_idioms)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod http_connector;
use http_connector::{RequestManager};
mod common;
use common::types::Result;
mod data_storage;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let now = Instant::now();
    let rq_mngr = RequestManager::new();

    rq_mngr.get_portfolio().await?;

    rq_mngr.get_market_bonds().await?;
    rq_mngr.get_market_currencies().await?;
    rq_mngr.get_market_etfs().await?;
    rq_mngr.get_market_stocks().await?;

    println!("\n\nExecution time {} ms.\n\n", now.elapsed().as_millis());

    Ok(())
}
