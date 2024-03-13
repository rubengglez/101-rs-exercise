use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// does testing things
    QuestionEntering {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    Quiz {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}


fn main() {
    
    println!("Hello, world!");
}
