use std::env;

fn main() {
    // A program that takes an integer from the terminal 

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./main <integer>");
        return;
    }

    let num_arg: i32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Wrong input, requires an integer");
            return;
        }
    };

    //Print out from zero to num_arg using loop, for in and while loop
    let mut numero: i32 = 0;
    loop {
        if numero == num_arg {
            break
        } else {
            println!("{numero}");
            numero += 1;
        }
    }
    println!("");
    // // Using a for in loop
    for i in 0..num_arg {
        println!("{i}");
    }

    println!("");
    //using a while loop
    let mut x = 0;
    while x != num_arg {
        println!("{x}");
        x += 1;
    }
}