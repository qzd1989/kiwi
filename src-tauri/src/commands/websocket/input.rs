use super::utils::{get_required_i32, get_required_point, get_required_string};
use crate::app::App;
use anyhow::Result;
use serde_json::Value;

pub fn click_left(_args: Value) -> Result<Value> {
    App::input().click_left();
    Ok(serde_json::to_value(None::<i32>)?)
}
pub fn click_right(_args: Value) -> Result<Value> {
    App::input().click_right();
    Ok(serde_json::to_value(None::<i32>)?)
}
pub fn press_left(_args: Value) -> Result<Value> {
    App::input().press_left();
    Ok(serde_json::to_value(None::<i32>)?)
}
pub fn press_right(_args: Value) -> Result<Value> {
    App::input().press_right();
    Ok(serde_json::to_value(None::<i32>)?)
}
pub fn release_left(_args: Value) -> Result<Value> {
    App::input().release_left();
    Ok(serde_json::to_value(None::<i32>)?)
}
pub fn release_right(_args: Value) -> Result<Value> {
    App::input().release_right();
    Ok(serde_json::to_value(None::<i32>)?)
}
pub fn move_absolute(args: Value) -> Result<Value> {
    let point = get_required_point(&args, "absolute_point")?;
    App::input().move_absolute(&point);
    Ok(serde_json::to_value(None::<i32>)?)
}
pub fn move_relative(args: Value) -> Result<Value> {
    let point = get_required_point(&args, "offset")?;
    App::input().move_relative(&point);
    Ok(serde_json::to_value(None::<i32>)?)
}
pub fn get_mouse_location(_args: Value) -> Result<Value> {
    let point = App::input().get_mouse_location();
    Ok(serde_json::to_value(point)?)
}
pub fn scroll_vertical(args: Value) -> Result<Value> {
    let length = get_required_i32(&args, "length")?;
    App::input().scroll_vertical(length);
    Ok(serde_json::to_value(None::<i32>)?)
}
pub fn scroll_horizontal(args: Value) -> Result<Value> {
    let length = get_required_i32(&args, "length")?;
    App::input().scroll_horizontal(length);
    Ok(serde_json::to_value(None::<i32>)?)
}
pub fn press_key(args: Value) -> Result<Value> {
    let key = get_required_string(&args, "key")?;
    App::input().press_key(&key)?;
    Ok(serde_json::to_value(None::<i32>)?)
}
pub fn release_key(args: Value) -> Result<Value> {
    let key = get_required_string(&args, "key")?;
    App::input().release_key(&key)?;
    Ok(serde_json::to_value(None::<i32>)?)
}
pub fn click_key(args: Value) -> Result<Value> {
    let key = get_required_string(&args, "key")?;
    App::input().click_key(&key)?;
    Ok(serde_json::to_value(None::<i32>)?)
}
pub fn input_text(args: Value) -> Result<Value> {
    let text = get_required_string(&args, "text")?;
    App::input().input_text(&text);
    Ok(serde_json::to_value(None::<i32>)?)
}
