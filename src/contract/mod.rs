use anyhow::Result;

mod solidity;
mod chaincode;

pub trait Contract {
    fn build(&self, path: &str) -> Result<()>;
}

pub fn new(typ: &str) -> Box<dyn Contract> {
     match typ {
        chaincode::Chaincode::TYPE_ID => Box::new(chaincode::Chaincode::new()),
        solidity::Solidity::TYPE_ID => Box::new(solidity::Solidity::new()),
        _ => unimplemented!("{}", typ)
    }
}
