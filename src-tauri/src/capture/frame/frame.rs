use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use capture::Frame as CaptureFrame;
use image::{ImageBuffer, ImageEncoder, Rgba, RgbaImage};
use serde::Serialize;
use std::path::Path;

use crate::types::Base64Png;
#[derive(Debug, Clone, Serialize)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>, //format: RGBA
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
        let (width, height) = (self.width as u32, self.height as u32);
        if let Some(buffer) = ImageBuffer::from_raw(width, height, self.buffer.to_owned()) {
            Ok(buffer)
        } else {
            return Err(anyhow!("convert failed"));
        }
    }

    pub fn to_base64_png(&self) -> Result<Base64Png> {
        let mut png_data = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
        encoder
            .write_image(
                &self.buffer,
                self.width,
                self.height,
                image::ExtendedColorType::Rgba8,
            )
            .map_err(|error| anyhow!(error.to_string()))?;
        let base64_str = general_purpose::STANDARD.encode(&png_data);

        Ok(format!("data:image/png;base64,{}", base64_str))
    }

    pub fn save<P>(&self, path: P) -> Result<bool>
    where
        P: AsRef<Path>,
    {
        let (width, height) = (self.width as u32, self.height as u32);
        let image: RgbaImage = ImageBuffer::from_vec(width, height, self.buffer.clone()).unwrap();
        image.save(path)?;
        Ok(true)
    }
}

impl From<CaptureFrame> for Frame {
    fn from(value: CaptureFrame) -> Self {
        Frame::new(value.width, value.height, value.buffer)
    }
}
