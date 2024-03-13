use crate::errors::Errors;
use anyhow::{Ok, Result};

const MAX_NUMBER_AWSWERS: u8 = 4;

#[derive(Debug, Clone, Default)]
pub struct Question {
    question: String,
    answers: Vec<String>,
    right: u8,
}

impl Question {
    fn new(question: String) -> Self {
        Self {
            question,
            ..Default::default()
        }
    }

		fn is_completed(&self) -> bool {
			self.answers.len() == MAX_NUMBER_AWSWERS.into()
		}

    fn add_wrong_answer(&mut self, question: String) -> Result<()> {
        self.assert_max_not_reached()?;
        self.answers.push(question);
        Ok(())
    }

    fn add_right_answer(&mut self, question: String) -> Result<()> {
        if self.right != Default::default() {
            return Err(Errors::AlreadyRightAnswer.into());
        }
        self.answers.push(question);

        Ok(())
    }

    fn assert_max_not_reached(&self) -> Result<()> {
        if self.answers.len() == MAX_NUMBER_AWSWERS.into() {
            return Err(Errors::MaxNumberAnswersReached.into());
        }

        Ok(())
    }
}
