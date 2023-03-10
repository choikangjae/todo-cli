use clap::{Parser, Subcommand};
use serde_json;
use serde::{ Serialize, Deserialize };
use std::{fs::{self, File}, io::Write};

const FILE_LOCATION: &str = "./resources/todo";

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add { content: Option<String> },
    List,
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    done: bool,
    content: String,
    priority: u32,
}

impl Todo {
    fn new(content: &String) -> Self {
        Todo {
            done: false,
            content: content.to_string(),
            priority: 3,
        }
    }
}

fn init() -> File {
    //default settings
    fs::create_dir_all("./resources").expect("Creating directory was failed");
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(FILE_LOCATION)
        .unwrap()
}
fn retrieve_todos() -> Vec<Todo> {
    let todos = fs::read_to_string(FILE_LOCATION).expect("`File` or `Directory` not found");
    let todos: Vec<Todo> = if !todos.is_empty() {
        serde_json::from_str(&todos[..]).expect("Converting json to struct failed")
    } else {
        Vec::new()
    };
    return todos;
}
fn main() {
    let mut file = init();
    let args = Cli::parse();
    match &args.command {
        Commands::Add { content } => {
            match content {
                Some(content) => {
                    //create todo.
                    let todo = Todo::new(content);
                    let mut todos = retrieve_todos();
                    todos.push(todo);

                    //save them as vec.
                    let todos = serde_json::to_string(&todos).expect("Error ocurred during serialization");
                    file.write_all(todos.as_bytes()).unwrap();

                    println!("Content is added. {:?}", content)
                }
                None => println!("Please type a to-do you want to add. \n Example: {{{{todo add \"Something to do\"}}}}")
            }
        }
        Commands::List => {
            let todos = retrieve_todos();
            println!("{:?}", todos);
        }
    }
}
