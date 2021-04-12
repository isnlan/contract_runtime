use error::*;

mod chaincode;
mod solidity;

pub trait Contract {
    fn build(&self, path: &str) -> Result<()>;
}

pub fn new(typ: &str) -> Result<Box<dyn Contract>> {
    match typ {
        chaincode::Chaincode::TYPE_ID => Ok(Box::new(chaincode::Chaincode::new())),
        solidity::Solidity::TYPE_ID => Ok(Box::new(solidity::Solidity::new())),
        _ => Err(anyhow!("unsupported contract type")),
    }
}
