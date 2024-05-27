use std::env;

fn cli() {
    let args = env::args().collect::<Vec<String>>();

    match args[1].parse::<u32>() {
        Ok(n) => println!("{}", n),
        Err(_) => {
            println!("Wrong input");
            return;
        }
    }
}
