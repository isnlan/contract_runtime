use crate::kvledger::file_path;
use byteorder::WriteBytesExt;
use error::*;
use protos::Block;
use rocksdb::DB;
use std::io::Write;
use std::ops::Deref;
use std::path::{Path, PathBuf};

const LEDGER_KEY_PREFIX: u8 = b'l';

pub struct IDStore {
    db: DB,
}

impl IDStore {
    pub fn new(path: &str) -> Result<Self> {
        let path = file_path::ledger_provider_path(path);
        Ok(IDStore {
            db: rocksdb::DB::open_default(path)?,
        })
    }

    pub fn create_ledger_id(&self, ledger_id: &str, block: &Block) -> Result<()> {
        let key = self.encode_ledger_key(ledger_id);
        if self.db.get(&key)?.is_some() {
            return Err(anyhow!("ledger {:} exist", ledger_id));
        }

        self.db.put(key, utils::proto::marshal(block)?)?;
        Ok(())
    }

    pub fn ledger_id_exists(&self, ledger_id: &str) -> Result<bool> {
        let key = self.encode_ledger_key(&ledger_id);
        let v = self.db.get(key)?;
        Ok(v.is_some())
    }

    pub fn delete_ledger_id(&self, ledger_id: &str) -> Result<()> {
        let key = self.encode_ledger_key(ledger_id);
        self.db.delete(key)?;
        Ok(())
    }

    pub fn get_active_ledger_ids(&self) -> Result<Vec<String>> {
        let iter = self.db.iterator(rocksdb::IteratorMode::From(
            &vec![LEDGER_KEY_PREFIX],
            rocksdb::Direction::Forward,
        ));

        let list = iter
            .take_while(|(k, _)| k[0] == LEDGER_KEY_PREFIX)
            .map(|(k, _)| {
                let k = &k[1..];
                String::from_utf8(k.to_vec()).unwrap()
            })
            .collect::<Vec<String>>();
        Ok(list)
    }

    fn encode_ledger_key(&self, ledger_id: &str) -> Vec<u8> {
        let mut buf = vec![LEDGER_KEY_PREFIX];
        let _ = buf.write(ledger_id.as_bytes()).unwrap();
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::IDStore;
    use error::*;
    use protos::Block;
    use tempfile::TempDir;

    #[test]
    fn it_works() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let store = IDStore::new(temp_dir.path().to_str().unwrap())?;
        let blk = &Block {
            header: None,
            data: None,
            metadata: None,
        };
        store.create_ledger_id("chain1", &blk)?;
        store.create_ledger_id("chain2", &blk)?;
        store.create_ledger_id("chain3", &blk)?;

        assert!(store.ledger_id_exists("chain1")?);
        let list = store.get_active_ledger_ids()?;
        assert_eq!(list, vec!["chain1", "chain2", "chain3"]);

        store.delete_ledger_id("chain0")?;
        store.delete_ledger_id("chain1")?;
        assert!(!store.ledger_id_exists("chain1")?);

        Ok(())
    }
}
