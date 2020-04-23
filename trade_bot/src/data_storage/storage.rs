use std::str;
use bytes::Bytes;
use std::fs;

use crate::common::types::Result;
use crate::common;

pub struct Storages {
    storage: Option<&'static Storage>,
}

// Singleton structure
pub struct Storage {
}

pub static mut STORAGES: Storages = Storages {
    storage: Some(&Storage{}),
};


impl Storages {
    // Can only be used once, otherwise panic! Implementation of singleton.
    pub fn take_storage(&mut self) -> &Storage {
        let p = std::mem::replace(&mut self.storage, None);
        p.unwrap()
    }
}


impl Storage {
    pub async fn to_storage(&self, input: Vec<Bytes>, resp_kind: common::ResponseKind) -> Result<()> {
        let mut in_str = String::new();

        // TODO: Prolly there is no need to construct a str, better usefrom_slice, but need to concat chunks
        for chunk in input{
            in_str += str::from_utf8(&chunk)?;
        }
        let data = match resp_kind {
            common::ResponseKind::Portfolio => {serde_json::from_str::<common::Portfolio>(&in_str).unwrap()}
        };

        println!("{}", data.payload.positions[0].averagePositionPrice.currency.as_ref().unwrap());

        Ok(())
    }
}