use model::Question;

pub mod errors;
pub mod model;

pub fn get_questions() -> Vec<Question> {
    let mut v = Vec::new();
    let mut answers = Vec::new();
    answers.push(String::from("zero"));
    answers.push(String::from("uno"));
    answers.push(String::from("dos"));
    let q = Question {
        question: String::from("loloo"),
        answers,
        right: 2,
    };
    v.push(q);
    v.push(Question::default());
    v.push(Question::default());

    v
}
