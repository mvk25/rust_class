/// Create a program that takes in an input from the terminal and
/// a second one from the terminal , divides the first input from the second one.
/// Test for extreme conditions
/// Div by Zero
/// Integer Overflow
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <int1> <int2>", args[0]);
        return;
    }

    let num1: u32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Wrong input, requires an integer");
            return;
        }
    };

    let num2: u32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Wrong input, requires an integer");
            return;
        }
    };

    match divide_two_nums(num1, num2) {
        Some(result) => println!("{} / {} = {}", num1, num2, result),
        None => println!("Division by zero or integer overflow"),
    }
}

fn divide_two_nums(a: u32, b: u32) -> Option<u32> {
    if b == 0 {
        // println!("ZeroDivisionError");
        return None;
    }

    a.checked_div(b)
}