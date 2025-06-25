use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Progress {
    pub percentage: u32,
    pub message: String,
}
impl Progress {
    pub fn new(percentage: u32, message: String) -> Self {
        Self {
            percentage,
            message,
        }
    }

    pub fn start() -> Self {
        Self::default()
    }

    pub fn update(percentage: u32) -> Self {
        Self::new(percentage, String::from("running"))
    }

    pub fn finished() -> Self {
        Self::new(100, String::from("finished"))
    }
}
impl Default for Progress {
    fn default() -> Self {
        Self::new(0, String::from("start"))
    }
}
