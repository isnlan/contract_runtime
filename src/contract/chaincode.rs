use anyhow::Context;
use error::*;
use std::process::Command;

const GO_COMPILER: &str = "go";
const LDFLAGS: &str = "-w -extldflags \"-static\"";

pub struct Chaincode {}

impl Chaincode {
    pub const TYPE_ID: &'static str = "chaincode";

    pub fn new() -> Self {
        Chaincode {}
    }
}

impl super::Contract for Chaincode {
    fn build(&self, path: &str) -> Result<()> {
        let binary = std::path::Path::new(path).join(".build/app");

        let output = Command::new(GO_COMPILER)
            .arg("build")
            .arg("-ldflags")
            .arg(LDFLAGS)
            .arg("-o")
            .arg(binary.clone())
            .arg(path)
            .current_dir(path)
            .output()
            .with_context(|| "failed to compile source")?;
        if !output.stderr.is_empty() {
            return Err(anyhow!(String::from_utf8(output.stderr)?));
        }

        info!("build chaincode {:?} success!", binary);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Chaincode;
    use crate::contract::Contract;
    use std::env;

    #[test]
    fn test_build() {
        let path = env::current_dir().unwrap();
        let path = path.join("data/hellogo");
        println!("current dir {:?}", path.to_str().unwrap());
        let sol = Chaincode {};
        sol.build(path.to_str().unwrap()).unwrap()
    }
}
