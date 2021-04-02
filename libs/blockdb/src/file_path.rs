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
pub fn block_store_path(root_fs_path: &PathBuf, ledger_id: &str) -> PathBuf {
    root_fs_path
        .join("chains")
        .join(ledger_id)
        .join("blk-store")
}

pub fn chains_path(root_fs_path: &PathBuf) -> PathBuf {
    root_fs_path.join("chains")
}
