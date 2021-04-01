use crate::store::Store;

use error::*;
use std::path::PathBuf;
use crate::file_path;

pub struct LevelDBBlockStoreProvider {
    path: std::path::PathBuf,

}

impl LevelDBBlockStoreProvider {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        LevelDBBlockStoreProvider{path:path.into()}
    }
}

impl crate::BlockStoreProvider for LevelDBBlockStoreProvider {
    type S = Store;

    fn create_block_store(&self, ledger_id: &str) -> Result<Self::S> {
       self.open_block_store(ledger_id)
    }

    fn open_block_store(&self, ledger_id: &str) -> Result<Self::S> {
        let path = file_path::block_store_path(&self.path, ledger_id);
        let s = Store::open(path)?;
        Ok(s)
    }

    fn exists(&self, ledger_id: &str) -> Result<bool> {
        let exists = utils::path::exists(file_path::chains_path(&self.path), ledger_id);
        Ok(exists)
    }

    fn list(&self) -> Result<Vec<String>> {
        utils::path::list_sub_dir(file_path::chains_path(&self.path))
    }
}
