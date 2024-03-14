use std::io::{self, stdin, Read};

use anyhow::Result;
use clap::{Parser, Subcommand};
use quizzer::{get_questions, model::Question};

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
    println!();
    for question in questions {
        println!("{}", question.question);
        println!();
        println!("The possible answers are:");
        for (i, answer) in question.answers.iter().enumerate() {
            println!("{}. --- {}", i, answer);
        }

        println!();
        println!("Write the number of the answer: ");
        let mut buffer :[u8; 1] = [0; 1];
        let mut stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_exact(&mut buffer)?;

        let read = std::str::from_utf8(&buffer)?;
        if question.right.to_string() == read.to_owned() {
            right_answers += 1;
        }
    }

    println!();
    println!();
    println!();
    println!("--------- RESULTS ----------------");
    println!();
    println!("You have guessed {} answers", right_answers);

    Ok(())
}

fn main() {
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::QuestionEntering) => {
            println!("Printing question");
        }
        Some(Commands::Quiz) => {
            let questions = get_questions();
            ask_questions_and_show_result(questions);
        }
        None => {
            panic!("You must give a valid mode")
        }
    }
}
