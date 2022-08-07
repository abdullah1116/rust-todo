mod todo;
use std::fs::write;
use std::panic::set_hook;
use std::process::exit;
use todo::Todo;

pub(crate) static DB_FILEPATH: &str = "db.txt";

fn main() {
    set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<String>() {
            println!("{}", s);
        }
    }));

    let first_var = std::env::args().nth(1);
    let action = match first_var {
        Some(ref str) => str.as_str(),
        None => {
            help();
            ""
        }
    };

    match action {
        "@clear_db" => {
            write(DB_FILEPATH, "").expect("can't clear db");
            println!("db cleared");
            exit(0);
        }
        _ => {}
    };

    let mut todo: Todo = Todo::read();

    let item = std::env::args().nth(2);

    match action {
        "list" => {
            todo.list();
        }

        "add" => {
            let item_str = item.expect("no item specified");

            match todo.add(&item_str) {
                Ok(_) => todo.save_with_message(format!("{} added", &item_str)),
                Err(err) => println!("{}", err),
            }
        }

        "remove" => {
            let item_str = item.expect("no item specified");

            match todo.remove(&item_str) {
                Ok(_) => todo.save_with_message(format!("{} removed", &item_str)),
                Err(err) => println!("{}", err),
            }
        }

        "complete" => {
            let item_str = item.expect("no item specified");

            match todo.complete(&item_str) {
                Ok(_) => todo.save_with_message(format!("{} completed", &item_str)),
                Err(err) => println!("{}", err),
            }
        }

        "uncomplete" => {
            let item_str = item.expect("no item specified");

            match todo.uncomplete(&item_str) {
                Ok(_) => todo.save_with_message(format!("{} uncompleted", &item_str)),
                Err(err) => println!("{}", err),
            }
        }

        _ => help(),
    };
}

fn help() {
    println!("list\t\t\t list all todo");
    println!("add <todo name>\t\t add todo");
    println!("remove <todo name>\t remove todo");
    println!("complete <todo name>\t mark todo as completed");
    println!("uncomplete <todo name>\t mark todo as uncompleted");

    exit(0);
}
