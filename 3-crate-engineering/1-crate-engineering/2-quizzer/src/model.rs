use crate::errors::Errors;
use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

const MAX_NUMBER_AWSWERS: u8 = 4;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
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
        if self.right != 0 {
            return Err(Errors::AlreadyRightAnswer.into());
        }
        self.right = index;

        Ok(())
    }

    fn is_completed(&self) -> bool {
        self.answers.len() == <u8 as Into<usize>>::into(MAX_NUMBER_AWSWERS)
    }

    fn assert_max_not_reached(&self) -> Result<()> {
        if self.is_completed() {
            return Err(Errors::MaxNumberAnswersReached.into());
        }

        Ok(())
    }
}
