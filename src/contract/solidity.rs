use super::Contract;
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::process::Command;
use crate::utils;

pub struct SolidityContract {}

impl Contract for SolidityContract {
    fn build(&self, path: &str) -> Result<()> {
        let v = utils::glob_file_path(path, "sol")?;
        if v.is_empty() {
            return Err(anyhow!("source file not find"))
        }

        let output = Command::new("solc")
            .arg("--overwrite")
            .arg("--bin")
            .arg("--abi")
            .arg("-o")
            .arg(path)
            .arg(v.join(" "))
            .output()
            .expect("failed to compile source");
        if !output.stderr.is_empty() {
            return Err(anyhow!(String::from_utf8(output.stderr)?));
        }

        return Ok(());
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
        println!("current dir {:?}", path.to_str().unwrap());
        let sol = SolidityContract{};
        sol.build(path.to_str().unwrap()).unwrap()
    }
}
