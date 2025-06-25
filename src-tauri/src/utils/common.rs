use anyhow::{anyhow, Result};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
/// 在指定的目录中查找第一个匹配正则表达式的文件
///
/// # Arguments
///
/// * `pattern` - 例如：`"/path/kiwi-.*-py3-none-any\\.whl"`
///
/// # Returns
///
/// 返回找到的文件路径（第一个匹配），如果没找到则返回 `None`
pub fn find_matching_file(pattern: &str) -> Result<Option<PathBuf>> {
    let path = Path::new(pattern);
    let dir = path
        .parent()
        .ok_or(anyhow!("Invalid pattern: missing directory"))?;
    let filename_pattern = path
        .file_name()
        .ok_or(anyhow!("Invalid pattern: missing filename"))?
        .to_string_lossy();

    // 转换 shell 风格的通配符（*）为正则表达式（.*）
    // 如果用户输入的是正则，就直接用；如果是通配符可以加点转换。
    // 这里假设用户输入的就是正则。
    let regex = Regex::new(&filename_pattern)?;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        if regex.is_match(&file_name) {
            return Ok(Some(entry.path()));
        }
    }

    Ok(None)
}
