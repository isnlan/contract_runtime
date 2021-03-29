use envconfig::Envconfig;
use serde::Deserialize;

#[derive(Envconfig, Deserialize, Debug)]
pub struct Config {
    #[envconfig(from = "SERVER_ADDRESS", default = "0.0.0.0:8080")]
    pub server_address: String,
    #[envconfig(from = "DATA_PATH", default = "/Users/snlan/nfs-data/baas/contract-runtime")]
    pub data_path: String,
}
