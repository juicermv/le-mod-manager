use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct StoredLoadOrder {
    pub mod_list: Vec<StoredLoadOrderItem>
}

#[derive(PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct StoredLoadOrderItem {
    pub path: String,
    pub uid: u32,
    pub enabled: bool
}