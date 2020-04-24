use std::str;
use bytes::Bytes;
use std::cell::RefCell;

use crate::common::types::{Result, PortfolioStorageType, MarketStorageType};
use crate::common;
use crate::data_storage::storage;
use crate::data_storage::storage::StorageProcessor;



pub struct Updater{
    // Possibly holds different storages
    //storages: storage::Storages,
    portfolio_storage:  RefCell<storage::Storage<common::Portfolio, PortfolioStorageType>>,
    market_storage:  RefCell<storage::Storage<common::Instruments, MarketStorageType>>,
}

impl Updater {
    pub fn new() -> Updater {
        let mut _storages = storage::Storages::init();
        Updater {portfolio_storage: RefCell::new(_storages.portfolio_storage),
                 market_storage:    RefCell::new(_storages.market_storage)}
    }
    pub async fn write_json_to_storage(&self, input: Vec<Bytes>, resp_kind: common::ResponseKind) -> Result<()> {
        let mut full_v: Vec<u8> = Vec::new();
        for i in input {
            full_v.extend(i.iter());
        }

        let mut in_str = String::new();
        in_str += str::from_utf8(&full_v)?;

        // TODO: Make generic
        match resp_kind {
            common::ResponseKind::Portfolio => {self.write_portfolio(&in_str).await?},
            common::ResponseKind::Market =>    {self.write_market(&in_str).await?},
        };

        Ok(())
    }
    async fn write_market(&self, input: &str) -> Result<()> {
        let data = serde_json::from_str::<common::Market>(input).unwrap();
        self.market_storage.borrow_mut().data.write_to_storage(data.payload)?;

        Ok(())
    }
    
    async fn write_portfolio(&self, input: &str) -> Result<()> {
        let data = serde_json::from_str::<common::Portfolio>(&input).unwrap();
        self.portfolio_storage.borrow_mut().data.write_to_storage(data)?;

        Ok(())
    }
}