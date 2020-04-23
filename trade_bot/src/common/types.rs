//use hyper::Response;
use std::collections::HashMap;
use crate::common;

// A simple type alias so as to DRY.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
//pub type HttpResult = std::result::Result<Response<hyper::body::Body>, hyper::error::Error>;
// TODO: Key supposed to be a timestamp
pub type PortfolioStorage = HashMap<usize, common::Portfolio>;
