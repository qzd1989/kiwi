use crate::{capture::Frame, extensions::ImageBufferRgbaExt as _};
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use image::{ImageBuffer, Rgba};
use opencv::core::Mat;
pub type Base64Png = String;
pub trait Base64PngExt {
    fn to_buffer(&self) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>>;
    fn to_mat(&self) -> Result<Mat>;
    fn to_frame(&self) -> Result<Frame>;
}
impl Base64PngExt for Base64Png {
    fn to_buffer(&self) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let mut base64_str = self.as_str();
        let prefix = "data:image/png;base64,";
        if self.starts_with(prefix) {
            base64_str = &base64_str[prefix.len()..];
        }
        let bytes = general_purpose::STANDARD.decode(base64_str)?;
        let img = image::load_from_memory_with_format(&bytes, image::ImageFormat::Png)?;
        Ok(img.into_rgba8().into())
    }

    fn to_mat(&self) -> Result<Mat> {
        self.to_buffer()?.to_mat()
    }

    fn to_frame(&self) -> Result<Frame> {
        let buffer = self.to_buffer()?;
        Ok(Frame::new(buffer.width(), buffer.height(), buffer.to_vec()))
    }
}
