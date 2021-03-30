use super::Contract;
use anyhow::{Context, Result};

use crate::utils;
use std::process::Command;

const SOLIDITY_COMPILER: &str = "solc";
const ABI_GENERATOR: &str = "abigen";

pub struct SolidityContract {}

impl Contract for SolidityContract {
    fn build(&self, path: &str) -> Result<()> {
        let v = utils::glob_file_path(path, "sol")
            .with_context(|| format!("failed read source path {}", path))?;
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

        let bin_file =
            utils::glob_file_path(path, "bin").with_context(|| "failed generate *.bin file")?;
        if bin_file.is_empty() {
            return Err(anyhow!("*.bin file not generate"));
        }
        let abi_file =
            utils::glob_file_path(path, "abi").with_context(|| "failed generate *.abi file")?;
        if bin_file.is_empty() {
            return Err(anyhow!("*.abi file not generate"));
        }

        let path_entry = std::path::Path::new(path);
        let go_main_path = path_entry.join("main.go");
        let output = Command::new(ABI_GENERATOR)
            .arg("--bin")
            .arg(&bin_file[0])
            .arg("--abi")
            .arg(&abi_file[0])
            .arg("--pkg")
            .arg("main")
            .arg("--type")
            .arg("contract")
            .arg("--out")
            .arg(go_main_path)
            .output()
            .with_context(|| "failed to generate go file")?;

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
