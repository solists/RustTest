use std::str;
use bytes::Bytes;
use std::cell::RefCell;

use crate::common::types::{Result, PortfolioStorageType};
use crate::common;
use crate::data_storage::storage;
use crate::data_storage::storage::StorageProcessor;



pub struct Updater{
    // Possibly holds different storages
    //storages: storage::Storages,
    portfolio_storage:  RefCell<storage::Storage<common::Portfolio, PortfolioStorageType>>,
}

impl Updater {
    pub fn new() -> Updater {
        let mut _storages = storage::Storages::init();
        let mut _storage = _storages.take_storage();
        _storage.data.init();
        Updater {portfolio_storage: RefCell::new(_storage)}//, storages: _storages}
    }
    pub async fn write_json_to_storage(&self, input: Vec<Bytes>, resp_kind: common::ResponseKind) -> Result<()> {
        let mut in_str = String::new();
        // TODO: Prolly there is no need to construct a str, better usefrom_slice, but need to concat chunks
        for chunk in input {
            in_str += str::from_utf8(&chunk)?;
        }
        let data = match resp_kind {
            common::ResponseKind::Portfolio => {serde_json::from_str::<common::Portfolio>(&in_str).unwrap()}
        };

        self.portfolio_storage.borrow_mut().data.write_to_storage(data)?;
        self.portfolio_storage.borrow().data.write_to_file().await?;

        Ok(())
    }
}