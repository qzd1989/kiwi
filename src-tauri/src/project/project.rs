use super::Config;
use crate::app::App;
use crate::input::Token;
use crate::interpreter::Interpreter;
use anyhow::{anyhow, Error, Result};
use fs_extra::dir::{get_dir_content2, DirOptions};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

pub type VerifyStatusString = String; // valid, invalid, moved

pub struct Project {
    pub name: String,
    pub main_file: String,
    pub path: PathBuf,
    pub interpreter: Interpreter,
    pub config: Config,
}

pub enum VerifyStatus {
    Valid,
    Invalid,
    Moved,
}

impl Display for VerifyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            VerifyStatus::Valid => "valid",
            VerifyStatus::Invalid => "invalid",
            VerifyStatus::Moved => "moved",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub language: String,
    pub main_file: String,
    pub path: String,
    pub kiwi_version: String,
}

impl From<&Project> for ProjectInfo {
    fn from(project: &Project) -> Self {
        Self {
            name: project.name.clone(),
            language: project.interpreter.to_string(),
            main_file: project.main_file.clone(),
            path: project.path.to_str().unwrap().to_string(),
            kiwi_version: project.interpreter.get_kiwi_version(),
        }
    }
}

impl Project {
    pub fn new_from_project_path(path: PathBuf) -> Result<Self> {
        let config = Config::new_from_toml(&path)?;
        let name = config.project.name.clone();
        let interpreter =
            Interpreter::new_from_language_and_project_path(&config.project.language, &path)?;
        let main_file = interpreter.get_main_file();
        Ok(Self {
            name,
            main_file,
            path,
            interpreter,
            config,
        })
    }

    pub fn verify(path: impl AsRef<Path>) -> VerifyStatus {
        // Verify config.toml
        let Ok(config) = Config::new_from_toml(&path) else {
            return VerifyStatus::Invalid;
        };
        // Verify that the configured interpreter is available for the project
        Interpreter::verify_in_project(&config.project.language, &path)
    }

    pub fn init(interpreter: &Interpreter) -> Result<()> {
        interpreter.init()?;
        Ok(())
    }

    pub fn get_edit_command(interpreter: &Interpreter) -> String {
        match interpreter {
            Interpreter::Python(_) => {
                #[cfg(target_os = "macos")]
                return "open -n -a \"/Applications/Visual Studio Code.app\" \"${projectFolder}\""
                    .to_string();
                #[cfg(target_os = "windows")]
                return "\"${resourceFolder}/editor/vscode/Code.exe\" \"${projectFolder}\""
                    .to_string();
            }
        }
    }

    pub fn get_kiwi_version(interpreter: &Interpreter) -> String {
        interpreter.get_kiwi_version()
    }

    pub fn generate_record_file(&self) -> Result<PathBuf> {
        let suffix = self.interpreter.get_suffix();
        let suffix_with_point = format!(".{}", suffix);
        let mut options = DirOptions::new();
        options.depth = 1;
        let Ok(data) = get_dir_content2(self.path.clone(), &options) else {
            return Err(anyhow!("read project dir failed."));
        };
        let project_dir_with_sep = format!("{}{}", self.path.to_str().unwrap(), MAIN_SEPARATOR);
        let max_num = data
            .files
            .iter()
            .filter_map(|file_path_str| {
                let file = file_path_str.replace(&project_dir_with_sep, "");
                if file.starts_with("record") && file.ends_with(&suffix_with_point) {
                    let middle = &file["record".len()..file.len() - &suffix_with_point.len()];
                    if let Ok(num) = middle.parse::<u32>() {
                        return Some(num);
                    }
                }
                None
            })
            .max_by_key(|&num| num);
        let record_id = if let Some(num) = max_num { num + 1 } else { 0 };
        let path = self
            .path
            .join(format!("record{}{}", record_id, suffix_with_point));
        Ok(path)
    }

    pub fn get_recorder_header_template(&self) -> String {
        self.interpreter.get_recorder_header_template()
    }

    pub fn get_recorder_token_handler(
        &self,
        error_handler: Box<dyn Fn(Error) + Send + Sync + 'static>,
    ) -> Box<dyn Fn(Arc<Mutex<Instant>>, Vec<Token>, Arc<Mutex<BufWriter<File>>>) + Send> {
        self.interpreter.get_recorder_token_handler(error_handler)
    }

    pub fn reveal_folder(&self) {
        let path = self.path.clone();
        thread::spawn(move || {
            #[cfg(target_os = "windows")]
            {
                let _ = Command::new("explorer").arg("/select,").arg(&path).spawn();
            }
            #[cfg(target_os = "macos")]
            {
                let _ = Command::new("open").arg("-R").arg(&path).spawn();
            }
        });
    }

    pub fn open_in_editor(&self) -> Result<bool> {
        let resource_dir = App::get_resource_dir();
        let project_folder = self.path.clone();
        let execute_command_string = Self::replace_vars(
            &self.config.project.edit_command,
            &resource_dir,
            &project_folder,
        );
        let args = shell_words::split(&execute_command_string)
            .map_err(|error| anyhow!(error.to_string()))?;

        if args.is_empty() {
            return Err(anyhow!("The edit command is invalid."));
        }

        let mut command = Command::new(&args[0]);

        if args.len() > 1 {
            command.args(&args[1..]);
        }

        Ok(command.spawn().is_ok())
    }

    fn replace_vars(command: &str, resource_dir: &PathBuf, project_folder: &PathBuf) -> String {
        let resource = Self::normalize_path(resource_dir);
        let project = Self::normalize_path(project_folder);
        command
            .replace("${resourceFolder}", &resource)
            .replace("${projectFolder}", &project)
    }

    fn normalize_path(path: &Path) -> String {
        let s = path.to_string_lossy();
        if let Some(stripped) = s.strip_prefix(r"\\?\") {
            stripped.to_string()
        } else {
            s.to_string()
        }
    }
}
