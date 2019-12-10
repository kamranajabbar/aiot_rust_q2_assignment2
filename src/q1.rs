use std::io;

pub fn run() {
    println!("\n ---------- Basic Calculator - Started ---------- \n");

	let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("Failed to read line");
    let mut function = String::new();
    io::stdin().read_line(&mut function).expect("Failed to read line");
	let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("Failed to read line");
    
    let nums = Numbers {
        num1: number1, 
        num2: number2,
    };
    
    println!("{} - {}", nums.num1, nums.num2);
    println!(" ---------- Basic Calculator - Ended ---------- \n");
}

#[derive(Debug)]
struct Numbers<T, U> {
    num1: T,
    num2: U,
}