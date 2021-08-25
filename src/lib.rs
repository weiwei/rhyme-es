use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub word: String,
    /// normalized frequency
    pub freq: f32,
    pub nsyl: u8,
}

pub type WordRepo = HashMap<String, Vec<Entry>>;
