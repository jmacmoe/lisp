use std::io::{self, Write};
mod parser;

fn main() {
    println!("Lispr Version 0.1.0");
    println!("Press Ctrl+C to Exit");

    loop {
        print!("lispr> ");
        io::stdout().flush()
            .expect("Failed to output prompt");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        println!("{}", input);
        let t = parser::parse_string(&input);
        match t {
            Some(value) => println!("{}", value.value),
            None => println!("Not a token.")
        }
                
            
    }
}
