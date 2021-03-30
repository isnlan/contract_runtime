use anyhow::Result;

mod solidity;
mod chaincode;

pub trait Contract {
    fn build(&self, path: &str) -> Result<()>;
}
