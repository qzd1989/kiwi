mod engine;
mod key;

pub use engine::*;
pub use enigo::agent::Token;
pub use enigo::{
    Axis::{Horizontal, Vertical},
    Button,
    Coordinate::{Abs, Rel},
    Direction::{Click, Press, Release},
};

/// 内部用的底层按键（KeyCode），不对前端暴露
pub use enigo::Key as SystemKey;

/// 业务层可用的按键类型（对前端暴露）
pub use key::Key;
