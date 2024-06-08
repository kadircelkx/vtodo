pub mod todos;

use colored::*;
use std::{io::{self, Write}, process};

use todos::{add_todo, delete_todo, edit_todo, done_todo, undone_todo, get_todos};

fn get_user_input_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    loop {
        print!("{}[2J", 27 as char); // Use the clear screen code instead of directly using the ANSI character to clear the screen
        println!("Todos");
        println!("{}", "-".repeat(20));

        let todos = get_todos();
        for todo in &todos {
            let todo_text = if todo.done {
                todo.text.green()
            } else {
                todo.text.red()
            };
            println!("{} {}", todo.id.to_string().bold().underline(), todo_text);
        }

        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let action = get_user_input_line();
        let action_parts: Vec<_> = action.split_whitespace().collect(); // You don't need to specify the type when using `collect`.
        if let Some(command) = action_parts.first() {
            match *command {
                "exit" => process::exit(0),
                "add" => {
                    let text = action_parts[1..].join(" "); // We can directly slice the array to create it instead of using split_at.
                    add_todo(text);
                },
                "delete" => {
                    let id = action_parts[1..].join(""); // We can directly slice the array to create it instead of using split_at.
                    delete_todo(id.parse().unwrap());
                },
                "edit" => {
                    let id = action_parts[1..].join(""); // We can directly slice the array to create it instead of using split_at.
                    let new_text = get_user_input_line();
                    edit_todo(id.parse().unwrap(), new_text);
                },
                "done" => {
                    let id = action_parts[1..].join(""); // We can directly slice the array to create it instead of using split_at.
                    done_todo(id.parse().unwrap());
                },
                "undone" => {
                    let id = action_parts[1..].join(""); // We can directly slice the array to create it instead of using split_at.
                    undone_todo(id.parse().unwrap());
                },
                _ => println!("Invalid command!"),
            }
        }
    }
}
