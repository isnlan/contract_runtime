use ledger::Initializer;
use ledger::statedb::VersionedDBRocksProvider;
use ledger::kvledger::kv_ledger_provider::Provider;
use ledger::ledger_mgmt::LedgerMgr;
use error::*;
use blockdb::provider::LevelDBBlockStoreProvider;

pub mod peer;

pub fn new() -> Result<peer::Peer<Provider<VersionedDBRocksProvider, LevelDBBlockStoreProvider>>>{
    // let ledger_mgmt::ledger_mgmt
    let init = Initializer {
        root_fs_path: "/var/blink/production".to_string(),
    };
    let vp = VersionedDBRocksProvider::new(&init.root_fs_path);
    let bsp = LevelDBBlockStoreProvider::new(&init.root_fs_path);
    let provider = Provider::new(init, vp, bsp)?;
    let mgr = LedgerMgr::new(provider);
    Ok(peer::Peer::new(mgr))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
