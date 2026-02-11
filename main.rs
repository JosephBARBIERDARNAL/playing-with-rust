// Rust Learning Exercises
// Main entry point to run all exercises

// Declare the exercises module
mod exercises;

use std::io::{self, Write};

fn main() {
    println!("====================================");
    println!("  Welcome to Rust Learning Path!");
    println!("====================================");
    println!();
    println!("You have 10 exercises to complete:");
    println!("  01. Ownership");
    println!("  02. Borrowing & References");
    println!("  03. Error Handling (Option & Result)");
    println!("  04. Iterators");
    println!("  05. Closures");
    println!("  06. Traits");
    println!("  07. Generics");
    println!("  08. Collections (HashMap & HashSet)");
    println!("  09. Pattern Matching");
    println!("  10. Lifetimes");
    println!();
    println!("Each exercise has:");
    println!("  - README.md: Concepts, examples, and hints");
    println!("  - mod.rs: Exercise code to complete");
    println!();

    loop {
        println!("====================================");
        println!("Enter exercise number (1-10), 'all' to run all, or 'quit' to exit:");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "quit" | "q" | "exit" => {
                println!("Happy learning!");
                break;
            }
            "all" => {
                run_all_exercises();
            }
            "1" => exercises::ex01_ownership::exercise(),
            "2" => exercises::ex02_borrowing_references::exercise(),
            "3" => exercises::ex03_error_handling::exercise(),
            "4" => exercises::ex04_iterators::exercise(),
            "5" => exercises::ex05_closures::exercise(),
            "6" => exercises::ex06_traits::exercise(),
            "7" => exercises::ex07_generics::exercise(),
            "8" => exercises::ex08_collections::exercise(),
            "9" => exercises::ex09_pattern_matching::exercise(),
            "10" => exercises::ex10_lifetimes::exercise(),
            _ => println!("Invalid input. Please enter 1-10, 'all', or 'quit'."),
        }
        println!();
    }
}

fn run_all_exercises() {
    println!("\nRunning all exercises...\n");

    exercises::ex01_ownership::exercise();
    exercises::ex02_borrowing_references::exercise();
    exercises::ex03_error_handling::exercise();
    exercises::ex04_iterators::exercise();
    exercises::ex05_closures::exercise();
    exercises::ex06_traits::exercise();
    exercises::ex07_generics::exercise();
    exercises::ex08_collections::exercise();
    exercises::ex09_pattern_matching::exercise();
    exercises::ex10_lifetimes::exercise();

    println!("\nAll exercises completed!");
}
