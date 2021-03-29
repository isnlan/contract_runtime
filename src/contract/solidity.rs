use super::Contract;
use anyhow::{Context, Result};

use crate::utils;
use std::process::Command;

const SOLIDITY_COMPILER: &str = "solc";

pub struct SolidityContract {}

impl Contract for SolidityContract {
    fn build(&self, path: &str) -> Result<()> {
        let v = utils::glob_file_path(path, "sol")
            .with_context(|| format!("Failed read source path {}", path))?;
        if v.is_empty() {
            return Err(anyhow!("source file not find"));
        }

        let output = Command::new(SOLIDITY_COMPILER)
            .arg("--overwrite")
            .arg("--bin")
            .arg("--abi")
            .arg("-o")
            .arg(path)
            .arg(v.join(" "))
            .output()
            .with_context(|| "failed to compile source")?;
        if !output.stderr.is_empty() {
            return Err(anyhow!(String::from_utf8(output.stderr)?));
        }

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use crate::contract::solidity::SolidityContract;
    use crate::contract::Contract;
    use std::env;

    #[test]
    fn test_build() {
        let path = env::current_dir().unwrap();
        let path = path.join("data/helloworld");
        println!("current dir {:?}", path.to_str().unwrap());
        let sol = SolidityContract {};
        sol.build(path.to_str().unwrap()).unwrap()
    }
}
