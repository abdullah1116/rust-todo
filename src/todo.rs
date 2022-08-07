use bool;
use std::collections::HashMap;
use std::fs::{read_to_string, write};

use crate::DB_FILEPATH;

pub(crate) struct Todo {
    db: HashMap<String, bool>,
}

impl Todo {
    pub(crate) fn read() -> Todo {
        let db_string = match read_to_string(DB_FILEPATH) {
            Ok(str) => str,
            Err(_) => String::new(),
        };

        let db_map = db_string
            .lines()
            .filter(|str| str.len() >= 2)
            .map(|str| (&str[1..], &str[0..1]))
            .map(|(key, value)| (String::from(key), value == "1"))
            .collect();

        Todo { db: db_map }
    }

    pub(crate) fn list(&mut self) {
        for (key, value) in &self.db {
            let status = if *value { "[X]" } else { "[ ]" };

            println!("{} {}", status, key);
        }
    }

    pub(crate) fn add(&mut self, key: &String) -> Result<bool, String> {
        match self.db.get(key) {
            Some(_) => Err(format!("{} already exist", key)),
            None => {
                self.db.insert(key.to_owned(), false);
                Ok(true)
            }
        }
    }

    pub(crate) fn remove(&mut self, key: &String) -> Result<bool, String> {
        match self.db.get(key) {
            Some(_) => {
                self.db.remove(key);
                Ok(true)
            }
            None => Err(format!("{} doesn't exist", key)),
        }
    }

    pub(crate) fn complete(&mut self, key: &String) -> Result<bool, String> {
        match self.db.get_mut(key) {
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

    pub(crate) fn uncomplete(&mut self, key: &String) -> Result<bool, String> {
        match self.db.get_mut(key) {
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

    pub(crate) fn save(self) -> Result<(), std::io::Error> {
        let mut db_string = String::new();
        for (key, value) in self.db {
            db_string.push_str(&format!("{}{}\n", value as i8, key));
        }

        write(DB_FILEPATH, db_string)
    }

    pub(crate) fn save_with_message(self, msg: String) {
        match self.save() {
            Ok(_) => println!("{}", msg),
            Err(err_msg) => println!("An error occurred: {}", err_msg),
        };
    }
}
