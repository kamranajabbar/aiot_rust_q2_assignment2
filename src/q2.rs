use std::io;
use std::io::prelude::*;

pub fn run() {

    loop {
        print!(">>>");
        io::stdout().flush().ok();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input");
        let input_val = input.trim();

        println!("{}", input_val);

        io::stdout().flush().ok();
    }

}