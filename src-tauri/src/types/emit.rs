use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Emit {
    data: String,
    time: f64,
}
impl Emit {
    pub fn new(data: String, time: f64) -> Self {
        Self { data, time }
    }
}
