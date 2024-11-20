///全局对象结构体
use crate::capture::Frame;
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use image::{imageops, ImageBuffer, Rgba};
#[cfg(not(all(windows, debug_assertions)))]
use opencv::core::{Mat, CV_8UC1, CV_8UC4};
#[cfg(not(all(windows, debug_assertions)))]
use opencv::prelude::*;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Point {
    #[pyo3(get, set)]
    pub x: f64,
    #[pyo3(get, set)]
    pub y: f64,
}

impl Point {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: impl Into<f64>, height: impl Into<f64>) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct WeightPoint {
    pub point: Point,
    pub weight: f64,
}

impl WeightPoint {
    pub fn new(point: Point, weight: f64) -> Self {
        Self { point, weight }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LocatingColor {
    pub point: Point,
    pub hex: HexColor,
}

impl LocatingColor {
    pub fn new(point: Point, hex: String) -> Self {
        Self { point, hex }
    }
}

pub struct LimitedQueue<T> {
    size: usize,
    queue: VecDeque<T>,
}

impl<T> LimitedQueue<T> {
    pub fn new(size: usize) -> Self {
        LimitedQueue {
            size,
            queue: VecDeque::with_capacity(size),
        }
    }
    pub fn push(&mut self, item: T) {
        if self.queue.len() >= self.size {
            self.queue.pop_front();
        }
        self.queue.push_back(item);
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        self.queue.get(index)
    }
}

pub type HexColor = String;
pub trait HexColorExt {
    fn to_rgb(&self) -> RgbColor;
    fn to_u32(&self) -> u32;
}

impl HexColorExt for HexColor {
    fn to_rgb(&self) -> RgbColor {
        let r = u8::from_str_radix(&self[1..3], 16).unwrap();
        let g = u8::from_str_radix(&self[3..5], 16).unwrap();
        let b = u8::from_str_radix(&self[5..7], 16).unwrap();
        RgbColor(r, g, b)
    }
    fn to_u32(&self) -> u32 {
        u32::from_str_radix(self.trim_start_matches('#'), 16).unwrap()
    }
}

pub type Base64Png = String;
pub trait Base64PngExt {
    fn to_buffer(&self) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>>;
    #[cfg(not(all(windows, debug_assertions)))]
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
        let cursor = std::io::Cursor::new(bytes);
        let img = image::load(cursor, image::ImageFormat::Png)?;
        Ok(img.into_rgba8().into())
    }

    #[cfg(not(all(windows, debug_assertions)))]
    fn to_mat(&self) -> Result<Mat> {
        self.to_buffer()?.to_mat()
    }

    fn to_frame(&self) -> Result<Frame> {
        let buffer = self.to_buffer()?;
        Ok(Frame::new(buffer.width(), buffer.height(), buffer.to_vec()))
    }
}

pub trait MatExt {
    fn to_buffer(&self) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>>;
}

#[cfg(not(all(windows, debug_assertions)))]
impl MatExt for Mat {
    fn to_buffer(&self) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let mat_data = self.data_bytes()?;
        let (width, height) = (self.cols(), self.rows());
        let buffer = mat_data.to_vec();
        let image_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            buffer,
        )
        .unwrap();
        Ok(image_buffer)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct RgbColor(pub u8, pub u8, pub u8);

impl RgbColor {
    pub fn range_compare(
        &self,
        offset_r: u8,
        offset_g: u8,
        offset_b: u8,
        target: &RgbColor,
    ) -> Option<(i16, i16, i16)> {
        let range = (
            (self.0 - offset_r)..=(self.0 + offset_r),
            (self.1 - offset_g)..=(self.1 + offset_g),
            (self.2 - offset_b)..=(self.2 + offset_b),
        );
        if range.0.contains(&target.0) && range.1.contains(&target.1) && range.2.contains(&target.2)
        {
            return Some((
                self.0 as i16 - target.0 as i16,
                self.1 as i16 - target.1 as i16,
                self.2 as i16 - target.2 as i16,
            ));
        } else {
            None
        }
    }
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }

    pub fn to_u32(&self) -> u32 {
        ((self.0 as u32) << 16) | ((self.1 as u32) << 8) | (self.2 as u32)
    }
}

pub trait ImageBufferRgbaExt {
    #[cfg(not(all(windows, debug_assertions)))]
    fn to_mat(&self) -> Result<Mat>;
    fn crop(
        &self,
        x: impl Into<u32>,
        y: impl Into<u32>,
        width: impl Into<u32>,
        height: impl Into<u32>,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>>;
    #[cfg(not(all(windows, debug_assertions)))]
    fn mask(&self) -> Result<Mat>;
}

impl ImageBufferRgbaExt for ImageBuffer<Rgba<u8>, Vec<u8>> {
    #[cfg(not(all(windows, debug_assertions)))]
    fn to_mat(&self) -> Result<Mat> {
        // 获取图像的宽度和高度
        let (width, height) = self.dimensions();
        // 获取图像缓冲区的原始数据
        let rbga_data = self.to_owned().into_raw();
        // 创建一个用于存储BGRA数据的向量，大小与原始RGBA数据相同
        let mut bgra_data = vec![0u8; rbga_data.len()];
        // 遍历RGBA数据，将其转换为BGRA格式
        for i in (0..rbga_data.len()).step_by(4) {
            bgra_data[i] = rbga_data[i + 2]; // B
            bgra_data[i + 1] = rbga_data[i + 1]; // G
            bgra_data[i + 2] = rbga_data[i]; // R
            bgra_data[i + 3] = rbga_data[i + 3]; // A
        }
        // 使用转换后的BGRA数据创建一个新的Mat对象
        let mut mat = unsafe { Mat::new_rows_cols(height as i32, width as i32, CV_8UC4) }?;
        // 获取Mat对象的数据指针
        let mat_data = mat.data_mut();
        // 将BGRA数据复制到Mat对象的数据指针指向的内存区域
        unsafe {
            std::ptr::copy_nonoverlapping(bgra_data.as_ptr(), mat_data as *mut u8, bgra_data.len());
        }
        // 返回转换后的Mat对象
        Ok(mat)
    }

    fn crop(
        &self,
        x: impl Into<u32>,
        y: impl Into<u32>,
        width: impl Into<u32>,
        height: impl Into<u32>,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        imageops::crop_imm(self, x.into(), y.into(), width.into(), height.into()).to_image()
    }

    #[cfg(not(all(windows, debug_assertions)))]
    fn mask(&self) -> Result<Mat> {
        let mut vec = Vec::new();
        for (_, _, pixel) in self.enumerate_pixels() {
            if pixel[3] == 0 {
                vec.push(0);
            } else {
                vec.push(255);
            }
        }
        let (width, height) = self.dimensions();
        let mut mat = unsafe { Mat::new_rows_cols(height as i32, width as i32, CV_8UC1) }?;
        unsafe {
            std::ptr::copy_nonoverlapping(vec.as_ptr(), mat.data_mut() as *mut u8, vec.len());
        }
        Ok(mat)
    }
}
