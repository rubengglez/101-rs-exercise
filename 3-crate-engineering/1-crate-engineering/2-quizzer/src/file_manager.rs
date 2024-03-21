use std::{
    fs::File,
    io::{Read, Write},
};

use anyhow::Result;

use crate::model::Question;

#[derive(Debug, Default)]
pub struct FileManager {}

impl FileManager {
    pub fn new() -> FileManager {
        FileManager {}
    }

    pub fn save(&self, questions: &Vec<Question>) -> Result<()> {
        let json_string = serde_json::to_string(&questions)?;

        self.store(&json_string)
    }

    pub fn get_questions(&self) -> Result<Vec<Question>> {
        let mut file = File::open("questions.json")?;
        let mut json_string = String::new();
        file.read_to_string(&mut json_string)
            .expect("Unable to read file");
        Ok(serde_json::from_str(&json_string)?)
    }

    fn store(&self, data: &str) -> Result<()> {
        let mut file = File::create("questions.json")?;
        file.write_all(data.as_bytes())?;

        Ok(())
    }
}
