use std::str;
use bytes::Bytes;
use std::cell::RefCell;

use crate::common::types::Result;
use crate::common;
use crate::data_storage::storage;




pub struct Updater{
    // Possibly holds different storages
    //storages: storage::Storages,
    storage:  RefCell<storage::Storage>,
}

impl Updater {
    pub fn new() -> Updater {
        let mut _storages = storage::Storages::init();
        let mut _storage = _storages.take_storage();
        _storage.init();
        Updater {storage: RefCell::new(_storage)}//, storages: _storages}
    }
    pub async fn write_json_to_storage(&self, input: Vec<Bytes>, resp_kind: common::ResponseKind) -> Result<()> {
        let mut in_str = String::new();
        // TODO: Prolly there is no need to construct a str, better usefrom_slice, but need to concat chunks
        for chunk in input{
            in_str += str::from_utf8(&chunk)?;
        }
        let data = match resp_kind {
            common::ResponseKind::Portfolio => {serde_json::from_str::<common::Portfolio>(&in_str).unwrap()}
        };

        self.storage.borrow_mut().write_to_storage(data).await?;
        self.storage.borrow().write_to_file().await?;

        //println!("{}", data.payload.positions[0].averagePositionPrice.currency.as_ref().unwrap());

        Ok(())
    }
}