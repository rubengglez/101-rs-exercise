use std::io::{self};

use anyhow::Result;
use clap::{Parser, Subcommand};
use quizzer::{file_manager::{self, FileManager}, model::Question};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// does testing things
    QuestionEntering,
    Quiz,
}

fn ask_questions_and_show_result(questions: Vec<Question>) -> Result<()> {
    let mut right_answers = 0;
    for question in questions {
        println!();
        println!("Question: {}", question.question);
        println!();
        println!("The possible answers are:");
        for (i, answer) in question.answers.iter().enumerate() {
            println!("{}. {}", i, answer);
        }

        println!();
        println!("Write the number of the answer: ");
        let mut user_input = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut user_input)?;

        if question.right.to_string() == user_input.to_owned().trim() {
            right_answers += 1;
        }
        println!();
    }

    println!();
    println!("--------- RESULTS ----------------");
    println!();
    println!("You have guessed {} answers", right_answers);

    Ok(())
}

fn add_questions(file_manager: &FileManager) -> Result<()> {
    println!("Note: write exit to stop entering questions");
    println!();

    let mut input = String::new();
    loop {
        println!("Write your question: ");
        io::stdin().read_line(&mut input)?;

        let data = input.trim();

        if data == "exit" {
            break;
        }
        println!();
        let mut question = Question::new(data.to_owned());
        input.clear();

        println!("Write the first awnswer: ");
        io::stdin().read_line(&mut input)?;
        let data = input.trim();

        if data == "exit" {
            break;
        }
        question.add_answer(data.to_owned())?;
        input.clear();
        println!();

        println!("Write the second awnswer: ");
        io::stdin().read_line(&mut input)?;
        let data = input.trim();

        if data == "exit" {
            break;
        }
        question.add_answer(data.to_owned())?;
        input.clear();
        println!();

        println!("Write the third awnswer: ");
        io::stdin().read_line(&mut input)?;
        let data = input.trim();

        if data == "exit" {
            break;
        }
        question.add_answer(data.to_owned())?;
        input.clear();
        println!();

        println!("Write the four awnswer: ");
        io::stdin().read_line(&mut input)?;
        let data = input.trim();

        if data == "exit" {
            break;
        }
        question.add_answer(data.to_owned())?;
        input.clear();
        println!();

        let data = input.trim();

        if data == "exit" {
            break;
        }

        println!();
        println!("What is the right answer? Write the number");
        for (i, answer) in question.answers.iter().enumerate() {
            println!("{}. {}", i, answer);
        }
        println!();
        io::stdin().read_line(&mut input)?;
        let data = input.trim();

        if data == "exit" {
            break;
        }
        question.set_right_answer(data.to_owned().parse::<u8>()?)?;
        file_manager.save(question);
        println!();
    }
    Ok(())
}

fn main() {
    let cli = Args::parse();
    let file_manager = FileManager::new();

    match &cli.command {
        Some(Commands::QuestionEntering) => {
            let _ = add_questions();
        }
        Some(Commands::Quiz) => {
            let questions = file_manager.get_questions().unwrap();
            ask_questions_and_show_result(questions).unwrap();
        }
        None => {
            panic!("You must give a valid mode")
        }
    }
}
