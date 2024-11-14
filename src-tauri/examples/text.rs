use image::DynamicImage;
use rusty_tesseract::{Args as TessArgs, Image as TessImage, TessResult};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Language {
    Chinese,
    English,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Chinese => "chi_sim",
            Language::English => "eng",
        }
    }
    pub fn all() -> Vec<Language> {
        let mut data = Vec::new();
        data.push(Self::Chinese);
        data.push(Self::English);
        data
    }
}

/// 识别图像中的文本
///
/// # 参数
/// - `image`: 一个引用类型的 `DynamicImage`，表示待识别文本的图像
/// - `langs`: 一个 `String` 表示用于文本识别的语言  "chi_sim+eng"
///
/// # 返回值
/// 返回一个 `TessResult<String>` 类型，其中 `Ok(String)` 表示成功识别的文本，
/// `Err(TessError)` 表示识别过程中发生的错误
pub fn recognize_text(image: &DynamicImage, langs: &str) -> TessResult<String> {
    // 将 `DynamicImage` 转换为 `TessImage`，以便于 Tesseract OCR 处理
    let image = TessImage::from_dynamic_image(&image)?;

    // 构建 Tesseract OCR 的识别参数
    let args = TessArgs {
        lang: langs.to_string(),
        dpi: Some(72),         // 每英寸点数
        psm: Some(3),          // 页面分割模式，值为 3 表示自动检测
        oem: Some(3),          // OCR 引擎模式，值为 3 表示使用 LSTM 模型
        ..TessArgs::default()  // 其他默认参数
    };

    // 调用 Tesseract OCR 进行文本识别
    match rusty_tesseract::image_to_string(&image, &args) {
        Ok(output) => Ok(output), // 识别成功，返回识别的文本
        Err(err) => Err(err),     // 识别失败，返回错误信息
    }
}

fn main() {
    let image = image::open("a.png").unwrap();
    let result = recognize_text(&image, "chi_sim+eng");
    match result {
        Ok(text) => {
            dbg!(text);
        }
        Err(error) => {
            dbg!(error);
        }
    }
}
