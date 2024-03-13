use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("impossible to add a right answer because it was already given")]
    AlreadyRightAnswer,
    #[error("max number of answers reached")]
    MaxNumberAnswersReached,
}
