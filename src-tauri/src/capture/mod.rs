pub mod crabgrab;
pub use crabgrab as engine;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>,
}

impl Frame {
    pub fn new(width: impl Into<u32>, height: impl Into<u32>, buffer: Vec<u8>) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
            buffer,
        }
    }
}
