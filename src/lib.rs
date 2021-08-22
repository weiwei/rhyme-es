use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub word: String,
    /// normalized frequency
    pub freq: f32,
}

pub type WordRepo = HashMap<String, Vec<Entry>>;
