use anyhow::{anyhow, Result};
use rust_embed::EmbeddedFile;
use std::{
    borrow::Cow,
    env,
    fs::File,
    io::{Cursor, Write as _},
    path::PathBuf,
};
pub trait EmbeddedFileExt {
    fn to_cursor(self) -> Cursor<Cow<'static, [u8]>>;
    fn to_tempfile(self, file_path: &str) -> Result<PathBuf>;
}
impl EmbeddedFileExt for EmbeddedFile {
    fn to_cursor(self) -> Cursor<Cow<'static, [u8]>> {
        Cursor::new(self.data)
    }
    fn to_tempfile(self, file_path: &str) -> Result<PathBuf> {
        let cursor = Cursor::new(self.data);
        let path = env::temp_dir().join(file_path);
        let mut file = File::create(&path).map_err(|error| anyhow!("{}", error))?;
        file.write_all(cursor.get_ref())
            .map_err(|error| anyhow!("{}", error))?;
        Ok(path)
    }
}
