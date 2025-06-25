use serde::Serialize;

pub type CommandResult<T> = Result<T, CommandError>;

#[derive(Serialize, Clone, Debug)]
pub struct CommandError {
    message: String,
}

impl From<String> for CommandError {
    fn from(value: String) -> Self {
        CommandError { message: value }
    }
}

impl From<&str> for CommandError {
    fn from(value: &str) -> Self {
        CommandError {
            message: value.to_string(),
        }
    }
}

impl From<image::ImageError> for CommandError {
    fn from(value: image::ImageError) -> Self {
        CommandError {
            message: value.to_string(),
        }
    }
}

impl From<anyhow::Error> for CommandError {
    fn from(value: anyhow::Error) -> Self {
        value.to_string().into()
    }
}
