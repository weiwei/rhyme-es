/// TODO: make a real lib.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub word: String,
    /// normalized frequency
    pub freq: f32,
}
