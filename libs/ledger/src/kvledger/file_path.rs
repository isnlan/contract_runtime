use std::path::{Path, PathBuf};

/// ledger path tree
/// ledger
///     ledger-provider
///     chains
///         chain1
///             blk-store
///             history
///             statedb
///             index
///         chain2
///             ...

pub fn ledger_provider_path(root_fs_path: &str) -> PathBuf {
    Path::new(root_fs_path).join("ledger-provider")
}

pub fn chain_store_path(root_fs_path: &str) -> PathBuf {
    Path::new(root_fs_path).join("chains")
}

pub fn state_db_path(root_fs_path: &PathBuf, ledger_id: &str) -> PathBuf {
    root_fs_path.join("chains").join(ledger_id).join("statedb")
}
