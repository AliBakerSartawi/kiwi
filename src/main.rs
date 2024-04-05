use std::io::{stdin, stdout, Write};

use store::Store;

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
        let input = input.trim();
        
        if input == "exit" || input == "quit" {
            break;
        }

        handle_command(input, store);
    }
}

fn handle_command(input: &str, store: &mut Store) {
    // TODO: empty command or `help` shows help

    println!("You entered: {}", input);
}
