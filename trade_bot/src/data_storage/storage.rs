use crate::common::types::{Result, PortfolioStorage};
use crate::common;
use std::collections::HashMap;
use std::path::Path;
use std::io::{BufReader, BufWriter};
use std::fs::File;

pub struct Storages {
    storage: Option<Storage>,
}

// Singleton structure
pub struct Storage {
    portfolios: PortfolioStorage,
    counter: usize,
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

        Storages {storage: Some(Storage{portfolios: HashMap::new(), counter: 0})}
    }
    // Can only be used once, otherwise panic! Implementation of singleton.
    pub fn take_storage(&mut self) -> Storage {
        let p = std::mem::replace(&mut self.storage, None);
        p.unwrap()
    }
}


impl Storage {
    pub fn init(&mut self) -> Option<String> {
        let path = Path::new("data/portfolios.json");
        let f = match File::open(path)  {
            Err(_) => return None,
            Ok(f) => f
        };
        let br = BufReader::new(f);
        //loop {
            /*let mut chunk: Vec<u8> = Vec::new();
            if let Ok(0) | Err(_) = br.read_until('#' as u8, &mut chunk) {
                break;
            };*/
            let storage_kind = common::ResponseKind::Portfolio;
            self.portfolios = match storage_kind {
                    common::ResponseKind::Portfolio => serde_json::from_reader::<_, PortfolioStorage>(br).unwrap()
                };
            self.counter = self.portfolios.len();
        //}

        None

        //let contents = fs::read_to_string(path).expect("Error opening config.json");
    }
    pub async fn write_to_storage(&mut self, input: common::Portfolio) -> Result<()> {
        self.insert(input);

        Ok(())
    }

    pub async fn write_to_file(&self) -> serde_json::Result<()> {
        let path = Path::new("data/portfolios.json");
        let file = (File::create(&path)).unwrap();
        let bw = BufWriter::new(file);

        serde_json::to_writer(bw, &self.portfolios)
    }
    #[inline]
    fn insert(&mut self, input: common::Portfolio) {
        self.portfolios.insert(self.counter, input);
        self.counter += 1;
    }
}