// use core::num;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./main <int>");
        return;
    }

    let mut num_arg: i32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error. Use of Bad Types");
            return;
        }
    };

    //add one to the input
    num_arg = num_arg + 1;
    println!("{num_arg}");
    
    //minuses one from the input
    num_arg = num_arg - 1;
    println!("{num_arg}");

    // multiplies num_arg by two
    num_arg = num_arg * 2;
    println!("{num_arg}");

    //divides num_arg by 2
    num_arg = num_arg / 2;
    println!("{num_arg}");
}
