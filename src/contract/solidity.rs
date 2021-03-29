use super::Contract;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct SolidityContract {}

impl Contract for SolidityContract {
    fn build<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let dir = fs::read_dir(path)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::env;
    use crate::contract::solidity::SolidityContract;
    use crate::contract::Contract;

    #[test]
    fn test_build() {
        let path = env::current_dir().unwrap();
        let path = path.join("data/helloworld");
        println!("current dir {:?}", path);
        let sol = SolidityContract{};
        sol.build(path).unwrap()
    }
}
