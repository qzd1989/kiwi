use crate::input::SystemKey;
use anyhow::{anyhow, Error, Result};
use std::str::FromStr;

pub enum Key {
    Alt,
    Backspace,
    CapsLock,
    Control,
    Delete,
    DownArrow,
    End,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    Home,
    LeftArrow,
    Meta,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    Shift,
    Space,
    Tab,
    UpArrow,
    LeftBracket,
    RightBracket,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equal,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    SemiColon,
    Quote,
    BackSlash,
    Comma,
    Dot,
    Slash,
    KpMinus,
    KpPlus,
    KpMultiply,
    KpDivide,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDelete,
    BackQuote,
    Insert,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
}

impl From<Key> for SystemKey {
    fn from(value: Key) -> Self {
        match value {
            Key::Alt => SystemKey::Alt,
            Key::Backspace => SystemKey::Backspace,
            Key::CapsLock => SystemKey::CapsLock,
            Key::Control => SystemKey::Control,
            Key::Delete => SystemKey::Delete,
            Key::DownArrow => SystemKey::DownArrow,
            Key::End => SystemKey::End,
            Key::Escape => SystemKey::Escape,
            Key::F1 => SystemKey::F1,
            Key::F2 => SystemKey::F2,
            Key::F3 => SystemKey::F3,
            Key::F4 => SystemKey::F4,
            Key::F5 => SystemKey::F5,
            Key::F6 => SystemKey::F6,
            Key::F7 => SystemKey::F7,
            Key::F8 => SystemKey::F8,
            Key::F9 => SystemKey::F9,
            Key::F10 => SystemKey::F10,
            Key::F11 => SystemKey::F11,
            Key::F12 => SystemKey::F12,
            Key::F13 => SystemKey::F13,
            Key::F14 => SystemKey::F14,
            Key::F15 => SystemKey::F15,
            Key::F16 => SystemKey::F16,
            Key::F17 => SystemKey::F17,
            Key::F18 => SystemKey::F18,
            Key::F19 => SystemKey::F19,
            Key::F20 => SystemKey::F20,
            Key::Home => SystemKey::Home,
            Key::LeftArrow => SystemKey::LeftArrow,
            Key::Meta => SystemKey::Meta,
            Key::PageDown => SystemKey::PageDown,
            Key::PageUp => SystemKey::PageUp,
            Key::Return => SystemKey::Return,
            Key::RightArrow => SystemKey::RightArrow,
            Key::Shift => SystemKey::Shift,
            Key::Space => SystemKey::Space,
            Key::Tab => SystemKey::Tab,
            Key::UpArrow => SystemKey::UpArrow,
            Key::LeftBracket => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(33);

                #[cfg(target_os = "windows")]
                let key = SystemKey::OEM4;
                key
            }
            Key::RightBracket => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(30);

                #[cfg(target_os = "windows")]
                let key = SystemKey::OEM6;
                key
            }
            Key::Num1 => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(18);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Num1;
                key
            }
            Key::Num2 => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(19);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Num2;
                key
            }
            Key::Num3 => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(20);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Num3;
                key
            }
            Key::Num4 => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(21);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Num4;
                key
            }
            Key::Num5 => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(23);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Num5;
                key
            }
            Key::Num6 => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(22);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Num6;
                key
            }
            Key::Num7 => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(26);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Num7;
                key
            }
            Key::Num8 => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(28);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Num8;
                key
            }
            Key::Num9 => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(25);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Num9;
                key
            }
            Key::Num0 => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(29);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Num0;
                key
            }
            Key::Minus => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(27);

                #[cfg(target_os = "windows")]
                let key = SystemKey::OEMMinus;
                key
            }
            Key::Equal => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(24);

                #[cfg(target_os = "windows")]
                let key = SystemKey::OEMPlus;
                key
            }
            Key::Q => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(12);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Q;
                key
            }
            Key::W => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(13);

                #[cfg(target_os = "windows")]
                let key = SystemKey::W;
                key
            }
            Key::E => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(14);

                #[cfg(target_os = "windows")]
                let key = SystemKey::E;
                key
            }
            Key::R => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(15);

                #[cfg(target_os = "windows")]
                let key = SystemKey::R;
                key
            }
            Key::T => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(17);

                #[cfg(target_os = "windows")]
                let key = SystemKey::T;
                key
            }
            Key::Y => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(16);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Y;
                key
            }
            Key::U => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(32);

                #[cfg(target_os = "windows")]
                let key = SystemKey::U;
                key
            }
            Key::I => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(34);

                #[cfg(target_os = "windows")]
                let key = SystemKey::I;
                key
            }
            Key::O => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(31);

                #[cfg(target_os = "windows")]
                let key = SystemKey::O;
                key
            }
            Key::P => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(35);

                #[cfg(target_os = "windows")]
                let key = SystemKey::P;
                key
            }
            Key::A => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(0);

                #[cfg(target_os = "windows")]
                let key = SystemKey::A;
                key
            }
            Key::S => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(1);

                #[cfg(target_os = "windows")]
                let key = SystemKey::S;
                key
            }
            Key::D => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(2);

                #[cfg(target_os = "windows")]
                let key = SystemKey::D;
                key
            }
            Key::F => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(3);

                #[cfg(target_os = "windows")]
                let key = SystemKey::F;
                key
            }
            Key::G => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(5);

                #[cfg(target_os = "windows")]
                let key = SystemKey::G;
                key
            }
            Key::H => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(4);

                #[cfg(target_os = "windows")]
                let key = SystemKey::H;
                key
            }
            Key::J => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(38);

                #[cfg(target_os = "windows")]
                let key = SystemKey::J;
                key
            }
            Key::K => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(40);

                #[cfg(target_os = "windows")]
                let key = SystemKey::K;
                key
            }
            Key::L => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(37);

                #[cfg(target_os = "windows")]
                let key = SystemKey::L;
                key
            }
            Key::M => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(46);

                #[cfg(target_os = "windows")]
                let key = SystemKey::M;
                key
            }
            Key::Z => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(6);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Z;
                key
            }
            Key::X => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(7);

                #[cfg(target_os = "windows")]
                let key = SystemKey::X;
                key
            }
            Key::C => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(8);

                #[cfg(target_os = "windows")]
                let key = SystemKey::C;
                key
            }
            Key::V => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(9);

                #[cfg(target_os = "windows")]
                let key = SystemKey::V;
                key
            }
            Key::B => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(11);

                #[cfg(target_os = "windows")]
                let key = SystemKey::B;
                key
            }
            Key::N => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(45);

                #[cfg(target_os = "windows")]
                let key = SystemKey::N;
                key
            }
            Key::SemiColon => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(41);

                #[cfg(target_os = "windows")]
                let key = SystemKey::OEM1;
                key
            }
            Key::Quote => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(39);

                #[cfg(target_os = "windows")]
                let key = SystemKey::OEM7;
                key
            }
            Key::BackSlash => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(42);

                #[cfg(target_os = "windows")]
                let key = SystemKey::OEM5;
                key
            }
            Key::Comma => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(43);

                #[cfg(target_os = "windows")]
                let key = SystemKey::OEMComma;
                key
            }
            Key::Dot => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(47);

                #[cfg(target_os = "windows")]
                let key = SystemKey::OEMPeriod;
                key
            }
            Key::Slash => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(44);

                #[cfg(target_os = "windows")]
                let key = SystemKey::OEM2;
                key
            }
            Key::KpMinus => SystemKey::Subtract,
            Key::KpPlus => SystemKey::Add,
            Key::KpMultiply => SystemKey::Multiply,
            Key::KpDivide => SystemKey::Divide,
            Key::Kp0 => SystemKey::Numpad0,
            Key::Kp1 => SystemKey::Numpad1,
            Key::Kp2 => SystemKey::Numpad2,
            Key::Kp3 => SystemKey::Numpad3,
            Key::Kp4 => SystemKey::Numpad4,
            Key::Kp5 => SystemKey::Numpad5,
            Key::Kp6 => SystemKey::Numpad6,
            Key::Kp7 => SystemKey::Numpad7,
            Key::Kp8 => SystemKey::Numpad8,
            Key::Kp9 => SystemKey::Numpad9,
            Key::KpDelete => SystemKey::Delete,
            Key::BackQuote => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(50);

                #[cfg(target_os = "windows")]
                let key = SystemKey::OEM3;
                key
            }
            Key::Insert => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(114);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Insert;
                key
            }
            Key::PrintScreen => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(105);

                #[cfg(target_os = "windows")]
                let key = SystemKey::PrintScr;
                key
            }
            Key::ScrollLock => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(107);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Scroll;
                key
            }
            Key::Pause => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(110);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Pause;
                key
            }
            Key::NumLock => {
                #[cfg(target_os = "macos")]
                let key = SystemKey::Other(71);

                #[cfg(target_os = "windows")]
                let key = SystemKey::Numlock;
                key
            } //SystemKey::Function
              // "AltGr" => SystemKey::Alt,
              // "ControlRight" => SystemKey::Control,
              // "KpReturn" => SystemKey::Return,
              // "MetaRight" => SystemKey::Meta,
              // "ShiftRight" => SystemKey::Shift,
              // IntlBackslash
              // Unknown(u32),
              // F21
              // F22
              // F23
              // F24
              // _ => return Err(anyhow!("unsupported key")),
        }
    }
}

impl TryFrom<SystemKey> for Key {
    type Error = anyhow::Error;

    fn try_from(system_key: SystemKey) -> std::result::Result<Self, Self::Error> {
        let key = match system_key {
            SystemKey::Alt => Key::Alt,
            SystemKey::Backspace => Key::Backspace,
            SystemKey::CapsLock => Key::CapsLock,
            SystemKey::Control => Key::Control,
            SystemKey::Delete => Key::Delete,
            SystemKey::DownArrow => Key::DownArrow,
            SystemKey::End => Key::End,
            SystemKey::Escape => Key::Escape,
            SystemKey::F1 => Key::F1,
            SystemKey::F2 => Key::F2,
            SystemKey::F3 => Key::F3,
            SystemKey::F4 => Key::F4,
            SystemKey::F5 => Key::F5,
            SystemKey::F6 => Key::F6,
            SystemKey::F7 => Key::F7,
            SystemKey::F8 => Key::F8,
            SystemKey::F9 => Key::F9,
            SystemKey::F10 => Key::F10,
            SystemKey::F11 => Key::F11,
            SystemKey::F12 => Key::F12,
            SystemKey::F13 => Key::F13,
            SystemKey::F14 => Key::F14,
            SystemKey::F15 => Key::F15,
            SystemKey::F16 => Key::F16,
            SystemKey::F17 => Key::F17,
            SystemKey::F18 => Key::F18,
            SystemKey::F19 => Key::F19,
            SystemKey::F20 => Key::F20,
            SystemKey::Home => Key::Home,
            SystemKey::LeftArrow => Key::LeftArrow,
            SystemKey::Meta => Key::Meta,
            SystemKey::PageDown => Key::PageDown,
            SystemKey::PageUp => Key::PageUp,
            SystemKey::Return => Key::Return,
            SystemKey::RightArrow => Key::RightArrow,
            SystemKey::Shift => Key::Shift,
            SystemKey::Space => Key::Space,
            SystemKey::Tab => Key::Tab,
            SystemKey::UpArrow => Key::UpArrow,
            SystemKey::Subtract => Key::KpMinus,
            SystemKey::Add => Key::KpPlus,
            SystemKey::Multiply => Key::KpMultiply,
            SystemKey::Divide => Key::KpDivide,
            SystemKey::Numpad0 => Key::Kp0,
            SystemKey::Numpad1 => Key::Kp1,
            SystemKey::Numpad2 => Key::Kp2,
            SystemKey::Numpad3 => Key::Kp3,
            SystemKey::Numpad4 => Key::Kp4,
            SystemKey::Numpad5 => Key::Kp5,
            SystemKey::Numpad6 => Key::Kp6,
            SystemKey::Numpad7 => Key::Kp7,
            SystemKey::Numpad8 => Key::Kp8,
            SystemKey::Numpad9 => Key::Kp9,
            #[cfg(target_os = "macos")]
            SystemKey::Other(33) => Key::LeftBracket,
            #[cfg(target_os = "windows")]
            SystemKey::OEM4 => Key::LeftBracket,

            #[cfg(target_os = "macos")]
            SystemKey::Other(30) => Key::RightBracket,
            #[cfg(target_os = "windows")]
            SystemKey::OEM6 => Key::RightBracket,

            #[cfg(target_os = "macos")]
            SystemKey::Other(18) => Key::Num1,
            #[cfg(target_os = "windows")]
            SystemKey::Num1 => Key::Num1,

            #[cfg(target_os = "macos")]
            SystemKey::Other(19) => Key::Num2,
            #[cfg(target_os = "windows")]
            SystemKey::Num2 => Key::Num2,

            #[cfg(target_os = "macos")]
            SystemKey::Other(20) => Key::Num3,
            #[cfg(target_os = "windows")]
            SystemKey::Num3 => Key::Num3,

            #[cfg(target_os = "macos")]
            SystemKey::Other(21) => Key::Num4,
            #[cfg(target_os = "windows")]
            SystemKey::Num4 => Key::Num4,

            #[cfg(target_os = "macos")]
            SystemKey::Other(23) => Key::Num5,
            #[cfg(target_os = "windows")]
            SystemKey::Num5 => Key::Num5,

            #[cfg(target_os = "macos")]
            SystemKey::Other(22) => Key::Num6,
            #[cfg(target_os = "windows")]
            SystemKey::Num6 => Key::Num6,

            #[cfg(target_os = "macos")]
            SystemKey::Other(26) => Key::Num7,
            #[cfg(target_os = "windows")]
            SystemKey::Num7 => Key::Num7,

            #[cfg(target_os = "macos")]
            SystemKey::Other(28) => Key::Num8,
            #[cfg(target_os = "windows")]
            SystemKey::Num8 => Key::Num8,

            #[cfg(target_os = "macos")]
            SystemKey::Other(25) => Key::Num9,
            #[cfg(target_os = "windows")]
            SystemKey::Num9 => Key::Num9,

            #[cfg(target_os = "macos")]
            SystemKey::Other(29) => Key::Num0,
            #[cfg(target_os = "windows")]
            SystemKey::Num0 => Key::Num0,

            #[cfg(target_os = "macos")]
            SystemKey::Other(27) => Key::Minus,
            #[cfg(target_os = "windows")]
            SystemKey::OEMMinus => Key::Minus,

            #[cfg(target_os = "macos")]
            SystemKey::Other(24) => Key::Equal,
            #[cfg(target_os = "windows")]
            SystemKey::OEMPlus => Key::Equal,

            #[cfg(target_os = "macos")]
            SystemKey::Other(12) => Key::Q,
            #[cfg(target_os = "windows")]
            SystemKey::Q => Key::Q,

            #[cfg(target_os = "macos")]
            SystemKey::Other(13) => Key::W,
            #[cfg(target_os = "windows")]
            SystemKey::W => Key::W,

            #[cfg(target_os = "macos")]
            SystemKey::Other(14) => Key::E,
            #[cfg(target_os = "windows")]
            SystemKey::E => Key::E,

            #[cfg(target_os = "macos")]
            SystemKey::Other(15) => Key::R,
            #[cfg(target_os = "windows")]
            SystemKey::R => Key::R,

            #[cfg(target_os = "macos")]
            SystemKey::Other(17) => Key::T,
            #[cfg(target_os = "windows")]
            SystemKey::T => Key::T,

            #[cfg(target_os = "macos")]
            SystemKey::Other(16) => Key::Y,
            #[cfg(target_os = "windows")]
            SystemKey::Y => Key::Y,

            #[cfg(target_os = "macos")]
            SystemKey::Other(32) => Key::U,
            #[cfg(target_os = "windows")]
            SystemKey::U => Key::U,

            #[cfg(target_os = "macos")]
            SystemKey::Other(34) => Key::I,
            #[cfg(target_os = "windows")]
            SystemKey::I => Key::I,

            #[cfg(target_os = "macos")]
            SystemKey::Other(31) => Key::O,
            #[cfg(target_os = "windows")]
            SystemKey::O => Key::O,

            #[cfg(target_os = "macos")]
            SystemKey::Other(35) => Key::P,
            #[cfg(target_os = "windows")]
            SystemKey::P => Key::P,

            #[cfg(target_os = "macos")]
            SystemKey::Other(0) => Key::A,
            #[cfg(target_os = "windows")]
            SystemKey::A => Key::A,

            #[cfg(target_os = "macos")]
            SystemKey::Other(1) => Key::S,
            #[cfg(target_os = "windows")]
            SystemKey::S => Key::S,

            #[cfg(target_os = "macos")]
            SystemKey::Other(2) => Key::D,
            #[cfg(target_os = "windows")]
            SystemKey::D => Key::D,

            #[cfg(target_os = "macos")]
            SystemKey::Other(3) => Key::F,
            #[cfg(target_os = "windows")]
            SystemKey::F => Key::F,

            #[cfg(target_os = "macos")]
            SystemKey::Other(5) => Key::G,
            #[cfg(target_os = "windows")]
            SystemKey::G => Key::G,

            #[cfg(target_os = "macos")]
            SystemKey::Other(4) => Key::H,
            #[cfg(target_os = "windows")]
            SystemKey::H => Key::H,

            #[cfg(target_os = "macos")]
            SystemKey::Other(38) => Key::J,
            #[cfg(target_os = "windows")]
            SystemKey::J => Key::J,

            #[cfg(target_os = "macos")]
            SystemKey::Other(40) => Key::K,
            #[cfg(target_os = "windows")]
            SystemKey::K => Key::K,

            #[cfg(target_os = "macos")]
            SystemKey::Other(37) => Key::L,
            #[cfg(target_os = "windows")]
            SystemKey::L => Key::L,

            #[cfg(target_os = "macos")]
            SystemKey::Other(6) => Key::Z,
            #[cfg(target_os = "windows")]
            SystemKey::Z => Key::Z,

            #[cfg(target_os = "macos")]
            SystemKey::Other(7) => Key::X,
            #[cfg(target_os = "windows")]
            SystemKey::X => Key::X,

            #[cfg(target_os = "macos")]
            SystemKey::Other(8) => Key::C,
            #[cfg(target_os = "windows")]
            SystemKey::C => Key::C,

            #[cfg(target_os = "macos")]
            SystemKey::Other(9) => Key::V,
            #[cfg(target_os = "windows")]
            SystemKey::V => Key::V,

            #[cfg(target_os = "macos")]
            SystemKey::Other(11) => Key::B,
            #[cfg(target_os = "windows")]
            SystemKey::B => Key::B,

            #[cfg(target_os = "macos")]
            SystemKey::Other(45) => Key::N,
            #[cfg(target_os = "windows")]
            SystemKey::N => Key::N,

            #[cfg(target_os = "macos")]
            SystemKey::Other(41) => Key::SemiColon,
            #[cfg(target_os = "windows")]
            SystemKey::OEM1 => Key::SemiColon,

            #[cfg(target_os = "macos")]
            SystemKey::Other(39) => Key::Quote,
            #[cfg(target_os = "windows")]
            SystemKey::OEM7 => Key::Quote,

            #[cfg(target_os = "macos")]
            SystemKey::Other(42) => Key::BackSlash,
            #[cfg(target_os = "windows")]
            SystemKey::OEM5 => Key::BackSlash,

            #[cfg(target_os = "macos")]
            SystemKey::Other(43) => Key::Comma,
            #[cfg(target_os = "windows")]
            SystemKey::OEMComma => Key::Comma,

            #[cfg(target_os = "macos")]
            SystemKey::Other(47) => Key::Dot,
            #[cfg(target_os = "windows")]
            SystemKey::OEMPeriod => Key::Dot,

            #[cfg(target_os = "macos")]
            SystemKey::Other(44) => Key::Slash,
            #[cfg(target_os = "windows")]
            SystemKey::OEM2 => Key::Slash,

            #[cfg(target_os = "macos")]
            SystemKey::Other(50) => Key::BackQuote,
            #[cfg(target_os = "windows")]
            SystemKey::OEM3 => Key::BackQuote,

            #[cfg(target_os = "macos")]
            SystemKey::Other(114) => Key::Insert,
            #[cfg(target_os = "windows")]
            SystemKey::Insert => Key::Insert,

            #[cfg(target_os = "macos")]
            SystemKey::Other(105) => Key::PrintScreen,
            #[cfg(target_os = "windows")]
            SystemKey::PrintScr => Key::PrintScreen,

            #[cfg(target_os = "macos")]
            SystemKey::Other(107) => Key::ScrollLock,
            #[cfg(target_os = "windows")]
            SystemKey::Scroll => Key::ScrollLock,

            #[cfg(target_os = "macos")]
            SystemKey::Other(110) => Key::Pause,
            #[cfg(target_os = "windows")]
            SystemKey::Pause => Key::Pause,

            #[cfg(target_os = "macos")]
            SystemKey::Other(71) => Key::NumLock,
            #[cfg(target_os = "windows")]
            SystemKey::Numlock => Key::NumLock,
            _ => return Err(anyhow!("unsupported key")),
        };
        Ok(key)
    }
}

impl FromStr for Key {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = match s {
            "Alt" => Key::Alt,
            "Backspace" => Key::Backspace,
            "CapsLock" => Key::CapsLock,
            "Control" => Key::Control,
            "Delete" => Key::Delete,
            "DownArrow" => Key::DownArrow,
            "End" => Key::End,
            "Escape" => Key::Escape,
            "F1" => Key::F1,
            "F2" => Key::F2,
            "F3" => Key::F3,
            "F4" => Key::F4,
            "F5" => Key::F5,
            "F6" => Key::F6,
            "F7" => Key::F7,
            "F8" => Key::F8,
            "F9" => Key::F9,
            "F10" => Key::F10,
            "F11" => Key::F11,
            "F12" => Key::F12,
            "F13" => Key::F13,
            "F14" => Key::F14,
            "F15" => Key::F15,
            "F16" => Key::F16,
            "F17" => Key::F17,
            "F18" => Key::F18,
            "F19" => Key::F19,
            "F20" => Key::F20,
            "Home" => Key::Home,
            "LeftArrow" => Key::LeftArrow,
            "Meta" => Key::Meta,
            "PageDown" => Key::PageDown,
            "PageUp" => Key::PageUp,
            "Return" => Key::Return,
            "RightArrow" => Key::RightArrow,
            "Shift" => Key::Shift,
            "Space" => Key::Space,
            "Tab" => Key::Tab,
            "UpArrow" => Key::UpArrow,
            "LeftBracket" => Key::LeftBracket,
            "RightBracket" => Key::RightBracket,
            "Num1" => Key::Num1,
            "Num2" => Key::Num2,
            "Num3" => Key::Num3,
            "Num4" => Key::Num4,
            "Num5" => Key::Num5,
            "Num6" => Key::Num6,
            "Num7" => Key::Num7,
            "Num8" => Key::Num8,
            "Num9" => Key::Num9,
            "Num0" => Key::Num0,
            "Minus" => Key::Minus,
            "Equal" => Key::Equal,
            "Q" => Key::Q,
            "W" => Key::W,
            "E" => Key::E,
            "R" => Key::R,
            "T" => Key::T,
            "Y" => Key::Y,
            "U" => Key::U,
            "I" => Key::I,
            "O" => Key::O,
            "P" => Key::P,
            "A" => Key::A,
            "S" => Key::S,
            "D" => Key::D,
            "F" => Key::F,
            "G" => Key::G,
            "H" => Key::H,
            "J" => Key::J,
            "K" => Key::K,
            "L" => Key::L,
            "Z" => Key::Z,
            "X" => Key::X,
            "C" => Key::C,
            "V" => Key::V,
            "B" => Key::B,
            "N" => Key::N,
            "SemiColon" => Key::SemiColon,
            "Quote" => Key::Quote,
            "BackSlash" => Key::BackSlash,
            "Comma" => Key::Comma,
            "Dot" => Key::Dot,
            "Slash" => Key::Slash,
            "KpMinus" => Key::KpMinus,
            "KpPlus" => Key::KpPlus,
            "KpMultiply" => Key::KpMultiply,
            "KpDivide" => Key::KpDivide,
            "Kp0" => Key::Kp0,
            "Kp1" => Key::Kp1,
            "Kp2" => Key::Kp2,
            "Kp3" => Key::Kp3,
            "Kp4" => Key::Kp4,
            "Kp5" => Key::Kp5,
            "Kp6" => Key::Kp6,
            "Kp7" => Key::Kp7,
            "Kp8" => Key::Kp8,
            "Kp9" => Key::Kp9,
            "KpDelete" => Key::KpDelete,
            "BackQuote" => Key::BackQuote,
            "Insert" => Key::Insert,
            "PrintScreen" => Key::PrintScreen,
            "ScrollLock" => Key::ScrollLock,
            "Pause" => Key::Pause,
            "NumLock" => Key::NumLock,
            _ => return Err(anyhow!("unsupport key")),
        };
        Ok(key)
    }
}
