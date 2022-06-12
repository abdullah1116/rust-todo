use bool;
use std::collections::HashMap;
use std::panic::set_hook;
use std::process::exit;
use std::str::FromStr;

static DB_FILEPATH: &str = "db.txt";

fn main() {
    set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<String>() {
            println!("{}", s);
        }
    }));

    let action: String;
    match std::env::args().nth(1) {
        Some(str) => action = str,
        None => {
            println!("list\t\t\t list all todo");
            println!("add <todo name>\t\t add todo");
            println!("remove <todo name>\t remove todo");
            println!("complete <todo name>\t mark todo as completed");
            println!("uncomplete <todo name>\t mark todo as uncompleted");

            exit(0);
        }
    };

    let mut todo: Todo = Todo::new().expect("invalid db File");

    if action == "list" {
        todo.list();
    } else if action == "add" {
        let item = std::env::args().nth(2).expect("no item specified");

        match todo.add(&item) {
            Ok(_) => todo.save_with_message(format!("{} added", &item)),
            Err(err) => println!("{}", err),
        }
    } else if action == "remove" {
        let item = std::env::args().nth(2).expect("no item specified");

        match todo.remove(&item) {
            Ok(_) => todo.save_with_message(format!("{} removed", &item)),
            Err(err) => println!("{}", err),
        }
    } else if action == "complete" {
        let item = std::env::args().nth(2).expect("no item specified");

        match todo.complete(&item) {
            Ok(_) => todo.save_with_message(format!("{} completed", &item)),
            Err(err) => println!("{}", err),
        }
    } else if action == "uncomplete" {
        let item = std::env::args().nth(2).expect("no item specified");

        match todo.uncomplete(&item) {
            Ok(_) => todo.save_with_message(format!("{} uncompleted", &item)),
            Err(err) => println!("{}", err),
        }
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let db_string: String;

        match std::fs::read_to_string(DB_FILEPATH) {
            Ok(str) => db_string = str,
            Err(_) => db_string = String::new(),
        }

        let db_map: HashMap<String, bool> = db_string
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();

        Ok(Todo { map: db_map })
    }

    fn list(&mut self) {
        for (key, value) in &self.map {
            let status = if *value { "Completed" } else { "Uncompleted" };

            println!("{}\t{}", key, status);
        }
    }

    fn add(&mut self, key: &String) -> Result<bool, String> {
        match self.map.get(key) {
            Some(_) => Err(format!("{} already exist", key)),
            None => {
                self.map.insert(key.to_owned(), false);
                Ok(true)
            }
        }
    }

    fn remove(&mut self, key: &String) -> Result<bool, String> {
        match self.map.get(key) {
            Some(_) => {
                self.map.remove(key);
                Ok(true)
            }
            None => Err(format!("{} doesn't exist", key)),
        }
    }

    fn complete(&mut self, key: &String) -> Result<bool, String> {
        match self.map.get_mut(key) {
            Some(item) => {
                if *item {
                    Err(format!("{} already completed", key))
                } else {
                    *item = true;
                    Ok(true)
                }
            }
            None => Err(format!("{} doesn't exist", key)),
        }
    }

    fn uncomplete(&mut self, key: &String) -> Result<bool, String> {
        match self.map.get_mut(key) {
            Some(item) => {
                if *item {
                    *item = false;
                    Ok(true)
                } else {
                    Err(format!("{} already uncompleted", key))
                }
            }
            None => Err(format!("{} doesn't exist", key)),
        }
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }

        std::fs::write(DB_FILEPATH, content)
    }

    fn save_with_message(self, msg: String) {
        match self.save() {
            Ok(_) => println!("{}", msg),
            Err(why) => println!("An error occurred: {}", why),
        };
    }
}
