use std::{
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
};

use dirs::config_local_dir;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Todo {
    pub text: String,
    pub id: i64,
    pub done: bool,
}

// Function to get the path of the todo list file
fn get_path() -> PathBuf {
    let mut path = config_local_dir().expect("Error: unable to resolve local .config dir.");
    path.push("vtodo.list");
    path
}

// Function to read todos from the file
fn read_todos() -> Result<Vec<Todo>, io::Error> {
    let path = get_path();
    if !path.exists() {
        File::create(&path)?;
        fs::write(&path, "[]")?;
    }
    let mut file = File::open(&path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;
    Ok(serde_json::from_str(&json)?)
}

// Function to write todos to the file
fn write_todos(todos: &[Todo]) -> Result<(), io::Error> {
    let path = get_path();
    let json = serde_json::to_string_pretty(&todos)?;
    fs::write(&path, json)?;
    Ok(())
}

// Function to get todos from the file
pub fn get_todos() -> Vec<Todo> {
    read_todos().unwrap_or_else(|err| {
        eprintln!("Error: unable to read todo list file: {}", err);
        Vec::new()
    })
}

// Function to add a new todo
pub fn add_todo(text: String) {
    let mut todos = get_todos();
    let last_todo_id = todos.last().map_or(0, |todo| todo.id);
    let new_id = last_todo_id + 1;
    let todo = Todo { text, id: new_id, done: false };
    todos.push(todo);
    if let Err(err) = write_todos(&todos) {
        eprintln!("Error: unable to write todo list to file: {}", err);
    }
}

// Function to edit a todo
pub fn edit_todo(id: i64, new_text: String) {
    let mut todos = get_todos();
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
        todo.text = new_text;
        if let Err(err) = write_todos(&todos) {
            eprintln!("Error: unable to write todo list to file: {}", err);
        }
    } else {
        eprintln!("Error: todo with id {} not found", id);
    }
}

// Function to delete a todo
pub fn delete_todo(id: i64) {
    let mut todos = get_todos();
    todos.retain(|todo| todo.id != id);
    if let Err(err) = write_todos(&todos) {
        eprintln!("Error: unable to write todo list to file: {}", err);
    }
}

// Function to mark a todo as done
pub fn done_todo(id: i64) {
    let mut todos = get_todos();
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
        todo.done = true;
        if let Err(err) = write_todos(&todos) {
            eprintln!("Error: unable to write todo list to file: {}", err);
        }
    } else {
        eprintln!("Error: todo with id {} not found", id);
    }
}

// Function to mark a todo as undone
pub fn undone_todo(id: i64) {
    let mut todos = get_todos();
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
        todo.done = false;
        if let Err(err) = write_todos(&todos) {
            eprintln!("Error: unable to write todo list to file: {}", err);
        }
    } else {
        eprintln!("Error: todo with id {} not found", id);
    }
}
