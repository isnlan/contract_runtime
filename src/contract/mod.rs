use anyhow::Result;

mod solidity;

pub trait Contract {
    fn build(&self, path: &str) -> Result<()>;
}
