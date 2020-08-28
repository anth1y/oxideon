use std::io::stdout;
use std::io::{self, Write};

fn main() {
    loop {
        println!("Oxideon> ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                println!("You typed: {}", input);
            }
            Err(error) => println!("error: {}", error),
        }
        stdout().flush().unwrap();
    }
}
