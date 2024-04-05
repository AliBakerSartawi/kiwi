use std::io::{stdin, stdout, Write};

use parser::{parse_input, Command};
use store::Store;

mod parser;
mod store;

fn main() {
    println!("Welcome to the Kiwi 🥝 shell!");
    println!("Type 'help' to see a list of commands");

    let mut store = Store::new();

    shell(&mut store);
}

fn shell(store: &mut Store) {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match handle_command(input, store) {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn handle_command(input: String, store: &mut Store) -> Result<(), String> {
    // TODO: empty command or `help` shows help

    match parse_input(input)? {
        Command::Set(key, value) => {
            store.set(key, value);
            println!("OK")
        }
        Command::Get(key) => match store.get(&key) {
            Some(value) => println!("{:?}", value),
            None => println!("Key not found"),
        },
        Command::Del(key) => match store.del(&key) {
            Some(value) => println!("Deleted value: {:?}", value),
            None => println!("Key not found"),
        },
        Command::Exit => std::process::exit(0),
        Command::Help => {
            println!("Commands:");
            println!("  set <type> <key> <value> - Set a key-value pair");
            println!("       |");
            println!("       └─ <type> can be one of: str, int, float, bool");
            println!("  get <key>                - Get the value associated with a key");
            println!("  del <key>                - Delete a key-value pair");
            println!("  exit                     - Exit the shell");
            println!("  help                     - Show this help message");
        }
        Command::Unknown(cmd) => eprintln!("Unknown command: {}", cmd),
        Command::Empty => (),
    };

    Ok(())
}
