use rust_embed::Embed;

#[cfg(target_os = "macos")]
#[derive(Embed)]
#[folder = "assets/"]
#[include = "paddle_ocr/*"]
// #[include = "app/config.toml"]
#[include = "python/packages/kiwi/pyproject.toml"]
// #[include = "editor/vscode_macos.zip"]
// #[include = "python/interpreter_macos.zip"]
// #[include = "python/wheels.zip"]
// #[include = "python/project_template.zip"]
pub struct Asset;

#[cfg(target_os = "windows")]
#[derive(Embed)]
#[folder = "assets/"]
#[include = "paddle_ocr/*"]
// #[include = "app/config.toml"]
#[include = "python/packages/kiwi/pyproject.toml"]
// #[include = "editor/vscode_windows.zip"]
// #[include = "python/interpreter_windows.zip"]
// #[include = "python/wheels.zip"]
// #[include = "python/project_template.zip"]
pub struct Asset;
