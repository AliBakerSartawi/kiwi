use std::io::{stdin, stdout, Write};

fn main() {
    println!("Welcome to the Kiwi 🥝 shell!");
    println!("Type 'help' to see a list of commands");

    shell();
}

fn shell() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input == "exit" || input == "quit" {
            break;
        }

        handle_command(input);
    }
}

fn handle_command(input: &str) {
    // TODO: empty command or `help` shows help

    println!("You entered: {}", input);
}
