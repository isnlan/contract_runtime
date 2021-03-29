use anyhow::Result;
use std::path::Path;

mod solidity;

pub trait Contract {
    fn build(&self, path: &str) -> Result<()> ;
}
