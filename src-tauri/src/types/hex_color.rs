use super::RgbColor;
pub type HexColor = String;
pub trait HexColorExt {
    fn to_rgb(&self) -> RgbColor;
    fn to_u32(&self) -> u32;
    fn empty() -> Self;
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
    fn empty() -> Self {
        "#000000".to_string()
    }
}
