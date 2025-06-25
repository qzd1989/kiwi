use anyhow::Result;
use image::{ImageBuffer, Rgba};
use opencv::core::{Mat, MatTraitConst as _, MatTraitConstManual as _};
pub trait MatExt {
    fn to_buffer(&self) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>>;
}
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
