use super::PythonKey;
use crate::input::{Abs, Button, Click, Horizontal, Key, Press, Rel, Release, Token, Vertical};
use anyhow::{anyhow, Result};
pub trait TokenExt {
    fn to_python_statement(&self) -> Result<String>;
}

impl TokenExt for Token {
    fn to_python_statement(&self) -> Result<String> {
        let python_script = match self {
            Token::Text(text) => format!("input_text(\"text={}\")", text),
            Token::Key(system_key, direction) => {
                let key: Key = system_key.to_owned().try_into()?;
                let python_key: PythonKey = key.try_into()?;
                let python_key_statement = format!("Key.{}", python_key);
                match direction {
                    Press => format!("press_key(key={})", python_key_statement),
                    Release => format!("release_key(key={})", python_key_statement),
                    Click => format!("click_key(key={})", python_key_statement),
                }
            }
            Token::Button(button, direction) => match (button, direction) {
                (Button::Left, Press) => format!("press_left()"),
                (Button::Left, Release) => format!("release_left()"),
                (Button::Left, Click) => format!("click_left()"),
                (Button::Right, Press) => format!("press_right()"),
                (Button::Right, Release) => format!("release_right()"),
                (Button::Right, Click) => format!("click_right()"),
                (_, _) => return Err(anyhow!("unsupport button action")),
            },
            Token::MoveMouse(x, y, coordinate) => match coordinate {
                Abs => format!("move_absolute(absolute_point=Point(x={}, y={}))", x, y),
                Rel => format!("move_relative(offset=Point(x={}, y={}))", x, y),
            },
            Token::Scroll(length, axis) => match axis {
                Horizontal => format!("scroll_horizontal(length={})", length),
                Vertical => format!("scroll_vertical(length={})", length),
            },
            _ => return Err(anyhow!("unsupport token type")),
        };
        Ok(python_script)
    }
}
