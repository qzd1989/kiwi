use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use image::{imageops, ImageBuffer, RgbImage, Rgba};
use opencv::core::{Mat, MatTrait as _, CV_8UC1, CV_8UC4};

use crate::types::{Base64Png, Point, Size};
pub trait ImageBufferRgbaExt {
    fn to_rgb(&self) -> Result<RgbImage>;
    fn to_base64png(&self) -> Result<Base64Png>;
    fn to_mat(&self) -> Result<Mat>;
    fn crop(&self, point: Point, size: Size) -> ImageBuffer<Rgba<u8>, Vec<u8>>;
    fn mask(&self) -> Result<Mat>;
}
impl ImageBufferRgbaExt for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn to_rgb(&self) -> Result<RgbImage> {
        match ImageBuffer::from_vec(
            self.width(),
            self.height(),
            self.as_raw()
                .chunks(4)
                .flat_map(|px| &px[..3])
                .copied()
                .collect(),
        ) {
            Some(rgb_image) => Ok(rgb_image),
            None => Err(anyhow!("convert rgba buffer to rgb failed")),
        }
    }
    /// too slow, don't use it.
    fn to_base64png(&self) -> Result<Base64Png> {
        let mut bytes = Vec::new();
        if let Err(error) = self.write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Png,
        ) {
            return Err(anyhow!("{}", error));
        }
        Ok(format!(
            "data:image/png;base64,{}",
            general_purpose::STANDARD.encode(bytes)
        ))
    }
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
    fn crop(&self, point: Point, size: Size) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        imageops::crop_imm(
            self,
            point.x as u32,
            point.y as u32,
            size.width as u32,
            size.height as u32,
        )
        .to_image()
    }
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
