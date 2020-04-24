use crate::common::types::{Result, PortfolioStorageType, MarketStorageType};
use crate::common;
use std::collections::HashMap;
use std::path::Path;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use async_trait::async_trait;


pub struct Storages {
    portfolio_storage: Option<Storage<common::Portfolio, PortfolioStorageType>>,
    market_storage:    Option<Storage<common::Market, MarketStorageType>>,
}

// Singleton structure
pub struct Storage<U, T: StorageProcessor<U>> {
    pub data: T,
}
static mut STORAGES_IS_ONCE: bool = false;

impl Storages {
    // TODO: Prevent initializing more than once, may couse invalid IO ops
    pub fn init() -> Storages {
        // Unsafe to access to static mutable
        unsafe {
            if STORAGES_IS_ONCE {
                panic!("Storages must be a singleton!");
            }
            
            STORAGES_IS_ONCE = true;
        }

        Storages {
            portfolio_storage: Some(Storage{data: HashMap::new()}),
            market_storage:    Some(Storage{data: HashMap::new()})
        }
    }
    // Can only be used once, otherwise panic! Implementation of singleton.
    pub fn take_storage(&mut self) -> Storage::<common::Portfolio, PortfolioStorageType> {
        let p = std::mem::replace(&mut self.portfolio_storage, None);
        p.unwrap()
    }
}

#[async_trait]
pub trait StorageProcessor<T> {
    fn init(&mut self) -> Option<String>;
    async fn write_to_file(&self) -> serde_json::Result<()>;
    fn insert(&mut self, input: T);
    fn write_to_storage(&mut self, input: T) -> Result<()> {
        self.insert(input);

        Ok(())
    }
}

#[async_trait]
impl StorageProcessor<common::Portfolio> for PortfolioStorageType {
    fn init(&mut self) -> Option<String> {
        let path = Path::new("data/portfolios.json");
        let f = match File::open(path)  {
            Err(_) => return None,
            Ok(f) => f
        };
        let br = BufReader::new(f);
        *self = serde_json::from_reader::<_, PortfolioStorageType>(br).unwrap();

        None
    }

    async fn write_to_file(&self) -> serde_json::Result<()> {
        let path = Path::new("data/portfolios.json");
        let file = (File::create(&path)).unwrap();
        let bw = BufWriter::new(file);

        serde_json::to_writer(bw, &self)
    }

    #[inline]
    fn insert(&mut self, input: common::Portfolio) {
        self.insert(self.len(), input);
    }
}

#[async_trait]
impl StorageProcessor<common::Market> for MarketStorageType {
    fn init(&mut self) -> Option<String> {
        let path = Path::new("data/portfolios.json");
        let f = match File::open(path)  {
            Err(_) => return None,
            Ok(f) => f
        };
        let br = BufReader::new(f);
        *self = serde_json::from_reader::<_, MarketStorageType>(br).unwrap();

        None
    }

    async fn write_to_file(&self) -> serde_json::Result<()> {
        let path = Path::new("data/portfolios.json");
        let file = (File::create(&path)).unwrap();
        let bw = BufWriter::new(file);

        serde_json::to_writer(bw, &self)
    }

    #[inline]
    fn insert(&mut self, input: common::structs::Market) {
        self.insert(self.len(), input);
    }
}