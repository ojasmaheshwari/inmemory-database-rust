extern crate multimap;

use multimap::MultiMap;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::io::{self, Write};

#[derive(Debug, FromPrimitive)]
enum Choice {
    Create,
    Select,
    Update,
    Delete,
    ViewAll,
}

const OPTIONS: [&str; 5] = [
    "Create a new key value pair",
    "Select a key value pair",
    "Update value of a key",
    "Delete a key value pair",
    "View all entries",
];

fn display_prompt() {
    println!("Please choose: ");

    for i in 0..OPTIONS.len() {
        println!("{}) {}", i + 1, OPTIONS[i]);
    }
}

fn take_input_and_verify() -> Choice {
    io::stdout().flush().expect("flush stdout buffer");
    print!("> ");
    io::stdout().flush().expect("flush stdout buffer");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("reading input from stdin");

    // Try to parse input into integer
    let choice: Option<i32> = match input.trim().parse::<i32>() {
        Ok(number) => Some(number),
        Err(_) => None,
    };

    if choice == None || choice.unwrap() < 1 || choice.unwrap() > OPTIONS.len().try_into().unwrap()
    {
        println!("Invalid input, please try again.");
        take_input_and_verify();
    }

    FromPrimitive::from_i32(choice.unwrap() - 1).expect("get Choice enum from i32")
}

fn create_database() -> MultiMap<String, String> {
    let database = MultiMap::new();
    database
}

fn view_all_entries(database: &MultiMap<String, String>) {
    for (key, values) in database {
        for value in values {
            println!("{} {}", key, value);
        }
    }
}

fn create_entry(database: &mut MultiMap<String, String>) {
    let mut key = String::new();
    let mut value = String::new();

    println!("Enter key: ");

    io::stdin().read_line(&mut key).expect("take key input");

    println!("Enter value: ");

    io::stdin().read_line(&mut value).expect("take value input");

    database.insert(key.trim().to_string(), value.trim().to_string());

    println!("Entry stored successfully!");
}

fn update_entry(database: &mut MultiMap<String, String>) {
    println!("Enter key: ");

    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("take key input");
    let key = key.trim();

    if !database.contains_key(key) {
        println!("No such key exists.");
        return;
    }

    let values = database.get_vec(key).unwrap();
    if values.len() > 1 {
        println!("Caution: The key you are trying to update has multiple values. Doing so is not recommended and will lead to only the first insertion value being updated.");
    }

    println!("Enter updated value: ");

    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("take value input");
    let value = value.trim();

    *database.get_mut(&key.to_string()).unwrap() = value.to_string();

    println!("Entry updated successfully.");
}

fn select_entry(database: &MultiMap<String, String>) {
    println!("Enter key: ");

    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("take key input");
    let key = key.trim();

    if !database.contains_key(key) {
        println!("No such key exists.");
        return;
    }

    let values = database.get_vec(key).unwrap();

    for value in values {
        println!("{} {}", key, value);
    }
}

fn delete_entry(database: &mut MultiMap<String, String>) {
    println!("Enter key: ");

    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("take key input");
    let key = key.trim();

    if !database.contains_key(key) {
        println!("No such key exists.");
        return;
    }

    database.remove(key);

    println!("Entry deleted successfully.");
}

fn program_loop(database: &mut MultiMap<String, String>) {
    display_prompt();
    let choice = take_input_and_verify();

    match choice {
        Choice::Create => {
            create_entry(database);
            program_loop(database);
        }
        Choice::Update => {
            update_entry(database);
            program_loop(database);
        }
        Choice::Select => {
            select_entry(database);
            program_loop(database);
        }
        Choice::Delete => {
            delete_entry(database);
            program_loop(database);
        }
        Choice::ViewAll => {
            view_all_entries(database);
            program_loop(database);
        }
    }
}

fn main() {
    println!("Hello, This is a simple in-memory database application written in Rust");

    let mut database = create_database();
    program_loop(&mut database);
}
