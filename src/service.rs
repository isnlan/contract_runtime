use crate::contract;
use anyhow::Result;

pub struct Service {}

impl Service {
    pub fn new() -> Self {
        Service {}
    }

    pub fn build(&self, contract_type: &str, path: &str) -> Result<()> {
        let c = contract::new(contract_type)?;
        c.build(path)?;
        Ok(())
    }
}
