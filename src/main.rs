use serde::Deserialize;
use serde::Serialize;
use serde_json::from_str;
use std::env;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize, Debug, Serialize)]
#[allow(dead_code)]
struct Todo {
    name: String,
    time_created: u64,
    done: bool,
}
#[derive(Deserialize, Debug, Serialize)]
#[allow(dead_code)]
struct TodoJson {
    name: String,
    todos: Vec<Todo>,
}

impl TodoJson {
    pub fn new(name: String) -> Self {
        Self {
            name,
            todos: Vec::new(),
        }
    }
}

fn savejson(json: TodoJson) {
    let test = serde_json::to_string_pretty(&json);
    let _ = fs::write("./todo.json", test.unwrap().as_bytes());
}

fn getjson() -> TodoJson {
    let file = fs::read_to_string("./todo.json");
    let json: TodoJson = from_str(&file.unwrap()).unwrap();
    json
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return println!("You have to do args dummy!");
    }

    match args[0].as_str() {
        "init" => {
            let test = TodoJson::new(args[1].clone());
            let test = serde_json::to_string_pretty(&test);
            let _ = fs::write("./todo.json", test.unwrap().as_bytes());

            println!("Created {} in this folder!", args[1]);
        }, "add" => {
            args.remove(0);
            let name = args.join(" ");

            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let mut json = getjson();

            json.todos.push(Todo{name: name.clone(), time_created: time, done: false});

            savejson(json);

            println!("Added {} to the todolist!", name);
        }, "finish" => {
            args.remove(0);
            let name = args.join(" ");

            let mut json = getjson();

            

            let mut updated = false;
            for i in json.todos.iter_mut() {
                if i.name == name {
                    i.done = !i.done;
                    updated = true;
                    break;
                }
            }

            if updated {
                println!("Updated {}", name);
                savejson(json);
            } else {
                println!("[ERROR] Cound't find {}", name)
            }
        }, "show" => {
            println!("Hello, this is your todolist for this projects");

            let json = getjson();
            
            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            for todo in json.todos {
                println!();
                println!("{}", todo.name);
                if todo.done {
                    println!("Completed!");
                } else {
                    let diff = time - todo.time_created;
                    let days = diff / 86400;
                    let hours = (diff / 3600) - days * 24;
                    let minutes = (diff / 60) - hours * 60;
                    println!("Created {days}day(s) {hours}hour(s) and {minutes}minute(s) ago");
                }

                
            }
        }, _ => {
            println!("Didn't understand you there buddy!")
        }
    }
}