
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stat {
    base_stat: u32,
    name: String
}