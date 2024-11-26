pub mod crabgrab;
use std::sync::{Arc, Mutex};

#[cfg(not(all(windows, debug_assertions)))]
use crate::common::ImageBufferRgbaExt;
use crate::utils::fs::write_file;
use anyhow::{anyhow, Result};
pub use crabgrab as engine;
use image::{ImageBuffer, Rgba};
use lazy_static::lazy_static;
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

pub fn listen_primary_display() {
    std::thread::spawn(|| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(engine::listen_primary_display(|frame| {
            FRAME.lock().unwrap().replace(frame);
            *IS_CAPTURING.lock().unwrap() = true;
        }));
    });
}

lazy_static! {
    pub static ref CAPTURE_SWITCH: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref IS_CAPTURING: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref FRAME: Arc<Mutex<Option<Frame>>> = Arc::new(Mutex::new(None));
}
