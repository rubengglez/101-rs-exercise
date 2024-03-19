use crate::errors::Errors;
use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

const MAX_NUMBER_AWSWERS: u8 = 4;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Question {
    pub question: String,
    pub answers: Vec<String>,
    pub right: u8,
}

impl Question {
    pub fn new(question: String) -> Self {
        Self {
            question,
            ..Default::default()
        }
    }

    pub fn add_answer(&mut self, question: String) -> Result<()> {
        self.assert_max_not_reached()?;
        self.answers.push(question);
        Ok(())
    }

    pub fn set_right_answer(&mut self, index: u8) -> Result<()> {
        if self.right != Default::default() {
            return Err(Errors::AlreadyRightAnswer.into());
        }
        self.right = index;

        Ok(())
    }

    fn is_completed(&self) -> bool {
        self.answers.len() == MAX_NUMBER_AWSWERS.into()
    }

    fn assert_max_not_reached(&self) -> Result<()> {
        if self.answers.len() == MAX_NUMBER_AWSWERS.into() {
            return Err(Errors::MaxNumberAnswersReached.into());
        }

        Ok(())
    }
}
