use super::Contract;
use anyhow::Result;

pub struct SolidityContract {

}

impl Contract for SolidityContract {
    fn build(&self, _path: &str) -> Result<()> {
        Ok(())
    }
}
