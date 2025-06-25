use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct RgbOffset {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbOffset {
    pub fn new(r: impl Into<u8>, g: impl Into<u8>, b: impl Into<u8>) -> Self {
        Self {
            r: r.into(),
            g: g.into(),
            b: b.into(),
        }
    }
}
