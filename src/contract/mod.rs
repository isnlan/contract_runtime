use anyhow::Result;
use std::path::Path;

mod solidity;

pub trait Contract {
    fn build<P: AsRef<Path>>(&self, path: P) -> Result<()> ;
}
