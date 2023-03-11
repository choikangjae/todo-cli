use clap::{Parser, Subcommand};
use serde_json;
use serde::{ Serialize, Deserialize };
use std::{fs::{self, File}, io::{Write, Read}};

const TODO_LOCATION: &str = "./resources/todo";
const ID_LOCATION: &str = "./resources/id";

#[derive(Parser, Debug)]
struct Cli {
#[command(subcommand)]
command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add { content: Option<String> },
    List,
    Del { id: Option<String> },
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
id: String,
    done: bool,
    content: String,
    priority: u32,
}

impl Todo {
    fn new(content: &String) -> Self {
        let (id, _) = read_id();
        increment_id(&id);
        Todo {
            id,
                done: false,
                content: content.to_string(),
                priority: 3,
        }
    }
}

fn increment_id(id: &String) {
    let id: u32 = id.parse().unwrap();
    let next_id = &(id + 1).to_string()[..];
    let mut new_file = File::create(ID_LOCATION).unwrap();
    write!(new_file, "{}", next_id).unwrap();
}

fn read_id() -> (String, File) {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(ID_LOCATION)
        .unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    (buf, f)
}

fn init() -> File {
    let (buf, mut f) = read_id();
    if buf.is_empty() {
        f.write_all(b"0").unwrap();
    }

    fs::create_dir_all("./resources").expect("Creating directory was failed");
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(TODO_LOCATION)
        .unwrap()
}

fn retrieve_todos() -> Vec<Todo> {
    let todos = fs::read_to_string(TODO_LOCATION).expect("`File` or `Directory` not found");
    let todos: Vec<Todo> = if !todos.is_empty() {
        serde_json::from_str(&todos[..]).expect("Converting json to struct failed")
    } else {
        Vec::new()
    };
    return todos;
}

fn save_todos(todos: &Vec<Todo>, file: &mut File) {
    let todos = serde_json::to_string(&todos).expect("Error ocurred during serialization");
    file.write_all(todos.as_bytes()).unwrap();
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
                    let id = todo.id.clone();
                    let mut todos = retrieve_todos();
                    todos.push(todo);

                    //save them as vec.
                    save_todos(&todos, &mut file);
                    println!("Content is added. id = {}, content = {:?}", id, content)
                }
                None => println!("Please type a to-do you want to add. \n Example: {{{{todo add \"Something to do\"}}}}")
            }
        }
        Commands::List => {
            let todos = retrieve_todos();
            println!("id---priority---content");
            for todo in todos {
                println!("{} {} {}", todo.id, todo.content, todo.priority);
            }
            println!("-----------------");
        }
        Commands::Del { id } => {
            match id {
                Some(id) => {
                    let mut todos = retrieve_todos();
                    let i = todos.iter().position(|todo| *todo.id == *id).unwrap();
                    todos.remove(i);

                    File::create(TODO_LOCATION).unwrap();
                    save_todos(&todos, &mut file);
                    println!("{id} is removed.");
                }
                None => println!("Please type id you want to delete. \n Example: {{{{todo del 1}}}}")
            }
        }
    }
}
