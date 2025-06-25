use serde::{Deserialize, Serialize};

use super::RgbOffset;
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct RgbColor(pub u8, pub u8, pub u8);
impl RgbColor {
    pub fn range_compare(
        &self,
        rgb_offset: RgbOffset,
        target: &RgbColor,
    ) -> Option<(i16, i16, i16)> {
        let (r, g, b) = (rgb_offset.r, rgb_offset.g, rgb_offset.b);
        let range = (
            (self.0.checked_sub(r).unwrap_or(0))..=(self.0.checked_add(r).unwrap_or(255)),
            (self.1.checked_sub(g).unwrap_or(0))..=(self.1.checked_add(g).unwrap_or(255)),
            (self.2.checked_sub(b).unwrap_or(0))..=(self.2.checked_add(b).unwrap_or(255)),
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
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2).to_lowercase()
    }
    pub fn to_u32(&self) -> u32 {
        ((self.0 as u32) << 16) | ((self.1 as u32) << 8) | (self.2 as u32)
    }
}
