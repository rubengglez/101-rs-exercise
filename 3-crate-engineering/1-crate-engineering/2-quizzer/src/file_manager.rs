use anyhow::Result;

use crate::model::Question;

#[derive(Debug, Default)]
pub struct FileManager {}

impl FileManager {
    pub fn new() -> FileManager {}

    pub fn save(question: Question) -> Result<()> {
        Ok(())
    }
}
