use crate::{capture::Frame, common::ImageBufferRgbaExt as _};
use anyhow::{anyhow, Result};
use image::DynamicImage;
use rusty_tesseract::{Args as TessArgs, Image as TessImage};

pub fn recognize(
    frame: Frame,
    langs: Vec<String>,
    x: impl Into<u32>,
    y: impl Into<u32>,
    width: impl Into<u32>,
    height: impl Into<u32>,
) -> Result<String> {
    let (x, y) = (x.into() as u32, y.into() as u32);
    let buffer = frame.to_buffer().unwrap().crop(x, y, width, height);
    let dynamic_image = DynamicImage::ImageRgba8(buffer);
    let image = TessImage::from_dynamic_image(&dynamic_image)?;
    let args = TessArgs {
        lang: langs.join("+"),
        dpi: Some(72),         // 每英寸点数
        psm: Some(3),          // 页面分割模式，值为 3 表示自动检测
        oem: Some(3),          // OCR 引擎模式，值为 3 表示使用 LSTM 模型
        ..TessArgs::default()  // 其他默认参数
    };
    rusty_tesseract::image_to_string(&image, &args).or_else(|error| Err(anyhow!(error.to_string())))
}
