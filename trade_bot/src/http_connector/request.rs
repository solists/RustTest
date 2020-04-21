use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};
use hyper::{Body, Method, Request};
use hyper::client::{HttpConnector};
use hyper_tls::HttpsConnector;

// A simple type alias so as to DRY.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_url(url: &hyper::Uri, connector: HttpsConnector<HttpConnector>, token: &str) -> Result<()> {
    let client = Client::builder().build::<_, hyper::Body>(connector);

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