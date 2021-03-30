use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Contract {
    pub name: String,
    pub contract_type: String,
    pub channel: String,
    pub path: String,
}
