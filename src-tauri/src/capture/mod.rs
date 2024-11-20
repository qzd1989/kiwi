pub mod crabgrab;
use anyhow::{anyhow, Result};
pub use crabgrab as engine;
use image::{ImageBuffer, Rgba};
use serde::Serialize;

use crate::common::ImageBufferRgbaExt;
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
    pub fn to_buffer(&self) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        if let Some(buffer) = ImageBuffer::from_raw(self.width, self.height, self.buffer.to_owned())
        {
            Ok(buffer)
        } else {
            return Err(anyhow!("Failed to convert frame to rgba buffer"));
        }
    }

    #[cfg(not(all(windows, debug_assertions)))]
    pub fn to_mat(&self) -> Result<opencv::core::Mat> {
        self.to_buffer()?.to_mat()
    }
}
