use crate::capture::Frame;
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use image::{imageops, ImageBuffer, Rgba};
use opencv::core::{Mat, CV_8UC1, CV_8UC4};
use opencv::prelude::*;
pub fn base64_to_rgba(base64_png_str: &str) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let mut base64_str = base64_png_str;
    let prefix = "data:image/png;base64,";
    if base64_png_str.starts_with(prefix) {
        base64_str = &base64_str[prefix.len()..];
    }
    let bytes = general_purpose::STANDARD.decode(base64_str)?;
    let cursor = std::io::Cursor::new(bytes);
    let img = image::load(cursor, image::ImageFormat::Png)?;
    Ok(img.into_rgba8().into())
}
pub fn rgba_to_mat(buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<Mat> {
    // 获取图像的宽度和高度
    let (width, height) = buffer.dimensions();
    // 获取图像缓冲区的原始数据
    let rbga_data = buffer.into_raw();
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

pub fn base64_to_mat(base64_png_str: &str) -> Result<Mat> {
    let buffer = base64_to_rgba(base64_png_str)?;
    rgba_to_mat(buffer)
}

pub fn crop_rgba(
    buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    x: impl Into<u32>,
    y: impl Into<u32>,
    width: impl Into<u32>,
    height: impl Into<u32>,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    imageops::crop_imm(buffer, x.into(), y.into(), width.into(), height.into()).to_image()
}

pub fn base64_to_frame(base64_png_str: &str) -> Result<Frame> {
    let buffer = base64_to_rgba(base64_png_str)?;
    Ok(Frame::new(buffer.width(), buffer.height(), buffer.to_vec()))
}

pub fn frame_to_rgba(frame: Frame) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    if let Some(image) = ImageBuffer::from_raw(frame.width, frame.height, frame.buffer) {
        Ok(image)
    } else {
        return Err(anyhow!("Failed to convert frame to rgba"));
    }
}

pub fn frame_to_mat(frame: Frame) -> Result<Mat> {
    let image = frame_to_rgba(frame)?;
    rgba_to_mat(image)
}

/// 将给定的带有透明度的图像缓冲区转换为二值图掩码。
///
/// 此函数用于将一个带有透明度的RGBA图像缓冲区转换成一个只有黑色（透明）和白色（不透明）的二值图。
/// 这个二值图可以作为模板匹配操作中的掩码使用，以忽略模板在匹配过程中的透明部分。
///
/// 参数:
/// buffer: ImageBuffer<Rgba<u8>, Vec<u8>>类型，包含RGBA像素值的图像缓冲区。
///
/// 返回:
/// Result<Mat, Box<dyn Error>>: 如果成功，返回一个代表二值图的Mat对象，否则返回一个错误类型。
///
/// 抛出:
/// 如果创建Mat对象失败，或者内存拷贝过程中出现错误，将返回一个错误类型。
pub fn mask(buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Result<Mat> {
    let mut vec = Vec::new();
    for (_, _, pixel) in buffer.enumerate_pixels() {
        if pixel[3] == 0 {
            vec.push(0);
        } else {
            vec.push(255);
        }
    }
    let (width, height) = buffer.dimensions();
    // 创建一个大小和图像尺寸相匹配的Mat对象，用于存储二值图。
    let mut mat = unsafe { Mat::new_rows_cols(height as i32, width as i32, CV_8UC1) }?;
    // 将处理后的向量数据拷贝到Mat对象中。
    unsafe {
        std::ptr::copy_nonoverlapping(vec.as_ptr(), mat.data_mut() as *mut u8, vec.len());
    }
    // 返回包含二值图的Mat对象。
    Ok(mat)
}

/// 将OpenCV的Mat对象转换为ImageBuffer<Rgba<u8>, Vec<u8>>类型的图像缓冲区
///
/// # 参数
/// * `mat` - 一个引用类型的Mat对象，包含图像数据
///
/// # 返回值
/// 返回一个Result类型，成功时包含转换后的图像缓冲区，失败时包含一个OpencvError错误类型
///
/// # 错误
/// 如果无法获取Mat对象的数据、宽度和高度转换失败、或者无法创建图像缓冲区，则返回错误
pub fn mat_to_rgba(mat: &Mat) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    // 获取Mat对象的数据
    let mat_data = mat.data_bytes()?;
    // 获取Mat对象的宽度和高度
    let (width, height) = (mat.cols(), mat.rows());
    // 将获取的数据复制到一个新的向量中
    let buffer = mat_data.to_vec();
    // 从原始数据创建图像缓冲区
    let image_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
        width.try_into().unwrap(),
        height.try_into().unwrap(),
        buffer,
    )
    .unwrap();
    // 返回成功的结果，包含图像缓冲区
    Ok(image_buffer)
}

// pub fn hex_to_u32(hex: &str) -> Result<u32, std::num::ParseIntError> {
//     u32::from_str_radix(hex.trim_start_matches('#'), 16)
// }

// pub fn hex_to_rgb(hex: &str) -> RgbColor {
//     let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
//     let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
//     let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
//     RgbColor(r, g, b)
// }
