/*extern crate hyper;

use std::time::{Instant, Duration};
use futures::executor::block_on;
use hyper::Client;

fn main() {
    let now = Instant::now();
    println!("1-{}", now.elapsed().as_millis());
    block_on(stopper(&now));
    println!("2-{}", now.elapsed().as_millis());
    
    //println!("{}", kek.poll(self: Pin<&mut Self>, cx: &mut Context<'_>));
}


async fn stopper(now: &Instant) -> i32 {
    println!("3-{}", now.elapsed().as_millis());
    let future = long();
    println!("4-{}", now.elapsed().as_millis());
    println!("{}-{}", future.await, now.elapsed().as_millis());

    42
}

async fn long() -> i32 {
    for i in 0..10000_0000 {
        //eprintln!("{} - skksk", i);
    }
    5
}*/

//#![deny(warnings)]
#![warn(rust_2018_idioms)]
use std::env;

use hyper::{body::HttpBody as _, Client};
use hyper::header::{AUTHORIZATION};
use tokio::io::{self, AsyncWriteExt as _};
use hyper_tls::HttpsConnector;
use hyper::{Body, Method, Request, Uri};

const token: &str = "--";

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    // Some simple CLI args requirements...
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return Ok(());
        }
    };

    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("https") {
        println!("This example only works with 'https' URLs.");
        return Ok(());
    }

    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let https = HttpsConnector::new();
    //let client = Client::new();
    let hdr = hyper::HeaderMap::new();
    
    let client = Client::builder().build::<_, hyper::Body>(https);

    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("content-type", "application/json")
        .header("Authorization", token)
        .body(Body::from(r#"{"library":"hyper"}"#))?;

    let mut res = client.request(req).await?;

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
