use std::path::{Path, PathBuf};

/// ledger path tree
/// ledger
///     ledger-provider
///     chains
///         chain1
///             blk-store
///             history
///             state
///             index
///         chain2
///             ...

pub fn ledger_provider_path(root_fs_path: &str) -> PathBuf {
    Path::new(root_fs_path).join("ledger-provider")
}

pub fn chain_store_path(root_fs_path: &str) -> PathBuf {
    Path::new(root_fs_path).join("chains")
}
