use crate::common::types::{Result, PortfolioStorageType, MarketStorageType};
use crate::common;
use std::collections::HashMap;
use std::path::Path;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use async_trait::async_trait;
use std::marker::PhantomData;
use std::io::BufRead;


pub struct Storages {
    pub portfolio_storage: Storage<common::Portfolio, PortfolioStorageType>,
    pub market_storage:    Storage<common::Instruments, MarketStorageType>,
}

// Singleton structure
pub struct Storage<U, T: StorageProcessor<U>> {
    pub        data: T,
    data_type: PhantomData<U>,
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

        let mut _s = Storages {
            portfolio_storage: Storage{data: HashMap::new(), data_type: PhantomData},
            market_storage:    Storage{data: HashMap::new(), data_type: PhantomData}
        };

        _s.market_storage.data.init();
        _s.portfolio_storage.data.init();

        _s
    }
}

impl<U, T: StorageProcessor<U>> Drop for Storage<U, T> {
    fn drop(&mut self) {
        self.data.write_to_file().unwrap();
    }
}

#[async_trait]
pub trait StorageProcessor<T> {
    fn init(&mut self) -> Option<String>;
    fn write_to_file(&self) -> serde_json::Result<()>;
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
        let mut br = BufReader::new(f);
        if br.fill_buf().unwrap().len() == 0 {
            return None;
        }
        *self = serde_json::from_reader::<_, PortfolioStorageType>(br).unwrap();

        None
    }

    fn write_to_file(&self) -> serde_json::Result<()> {
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
impl StorageProcessor<common::Instruments> for MarketStorageType {
    fn init(&mut self) -> Option<String> {
        let path = Path::new("data/markets.json");
        let f = match File::open(path)  {
            Err(_) => return None,
            Ok(f) => f
        };
        let mut br = BufReader::new(f);
        if br.fill_buf().unwrap().len() == 0 {
            return None;
        }
        *self = serde_json::from_reader::<_, MarketStorageType>(br).unwrap();

        None
    }

    fn write_to_file(&self) -> serde_json::Result<()> {
        let path = Path::new("data/markets.json");
        let file = (File::create(&path)).unwrap();
        let bw = BufWriter::new(file);

        serde_json::to_writer(bw, &self)
    }

    #[inline]
    fn insert(&mut self, input: common::Instruments) {
        for i in input.instruments {
            self.insert(i.figi.clone().unwrap(), i);
        }
    }
}