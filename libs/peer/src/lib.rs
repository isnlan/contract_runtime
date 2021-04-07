use blockdb::provider::LevelDBBlockStoreProvider;
use error::*;
use ledger::kvledger::kv_ledger_provider::Provider;
use ledger::ledger_mgmt::LedgerMgr;
use ledger::statedb::VersionedDBRocksProvider;
use ledger::Initializer;

pub mod peer;

pub fn new(
    init: Initializer,
) -> Result<peer::Peer<Provider<VersionedDBRocksProvider, LevelDBBlockStoreProvider>>> {
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
