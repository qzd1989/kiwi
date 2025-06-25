use super::Frame;
use crate::{
    extensions::{EmbeddedFileExt, ImageBufferRgbaExt as _},
    types::{Asset, Point, Size},
};
use anyhow::{anyhow, Result};
use paddle_ocr_rs::ocr_lite::OcrLite;
use std::sync::{Mutex, OnceLock};
impl Frame {
    pub fn recognize_text(&self, point: Point, size: Size) -> Result<Option<String>> {
        let rgb_image = self.to_buffer()?.crop(point, size).to_rgb()?;
        OCR.get_or_init(move || -> Mutex<OcrLite> {
            let ocr = Mutex::new(OcrLite::new());
            match (
                Asset::get("paddle_ocr/models/ch_PP-OCRv5_mobile_det.onnx"),
                Asset::get("paddle_ocr/models/ch_ppocr_mobile_v2.0_cls_infer.onnx"),
                Asset::get("paddle_ocr/models/ch_PP-OCRv5_rec_mobile_infer.onnx"),
            ) {
                (Some(det), Some(cls), Some(rec)) => {
                    let (det_bytes, cls_bytes, rec_bytes) =
                        (det.to_cursor(), cls.to_cursor(), rec.to_cursor());
                    let (det_bytes, cls_bytes, rec_bytes) = (
                        det_bytes.get_ref().as_ref(),
                        cls_bytes.get_ref().as_ref(),
                        rec_bytes.get_ref().as_ref(),
                    );
                    ocr.lock()
                        .unwrap()
                        .init_models_from_memory(det_bytes, cls_bytes, rec_bytes, 2)
                        .map_err(|error| {
                            let error_string = match error {
                                paddle_ocr_rs::ocr_error::OcrError::Ort(error) => {
                                    error.message().to_string()
                                }
                                paddle_ocr_rs::ocr_error::OcrError::Io(error) => error.to_string(),
                                paddle_ocr_rs::ocr_error::OcrError::ImageError(image_error) => {
                                    image_error.to_string()
                                }
                                paddle_ocr_rs::ocr_error::OcrError::SessionNotInitialized => {
                                    anyhow!("SessionNotInitialized").to_string()
                                }
                            };
                            anyhow!("init models from memory error.({})", error_string)
                        })
                        .unwrap();
                }
                _ => {}
            }
            ocr
        });
        let result = OCR
            .get()
            .unwrap()
            .lock()
            .unwrap()
            .detect(&rgb_image, 50, 1024, 0.5, 0.3, 1.6, true, false)?;
        let texts: Vec<String> = result
            .text_blocks
            .into_iter()
            .map(|text_block| text_block.text)
            .collect();
        let texts = texts.join("");
        if texts.is_empty() {
            return Ok(None);
        }
        Ok(Some(texts))
    }
}
//只想让它初始化一次
static OCR: OnceLock<Mutex<OcrLite>> = OnceLock::new();
