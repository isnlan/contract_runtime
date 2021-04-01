use anyhow::{Context, Result};

use std::process::Command;

const SOLIDITY_COMPILER: &str = "solc";
const ABI_GENERATOR: &str = "abigen";

pub struct Solidity {}

impl Solidity {
    pub const TYPE_ID: &'static str = "solidity";

    pub fn new() -> Self {
        Solidity {}
    }
}

impl super::Contract for Solidity {
    fn build(&self, path: &str) -> Result<()> {
        let v = utils::path::glob_file_path(path, "sol")
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
            utils::path::glob_file_path(path, "bin").with_context(|| "failed generate *.bin file")?;
        if bin_file.is_empty() {
            return Err(anyhow!("*.bin file not generate"));
        }

        let abi_file =
            utils::path::glob_file_path(path, "abi").with_context(|| "failed generate *.abi file")?;
        if bin_file.is_empty() {
            return Err(anyhow!("*.abi file not generate"));
        }

        let go_main_path = std::path::Path::new(path).join("main.go");
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
    use super::Solidity;
    use crate::contract::Contract;
    use std::env;

    #[test]
    fn test_build() {
        let path = env::current_dir().unwrap();
        let path = path.join("data/hellosol");
        println!("current dir {:?}", path.to_str().unwrap());
        let sol = Solidity {};
        sol.build(path.to_str().unwrap()).unwrap()
    }
}
