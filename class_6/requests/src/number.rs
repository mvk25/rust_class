pub fn numbers(x: u32) {
    match x {
        1 => println!("Number is 1 and 2 "),
        3..=10 => println!("Number is between 3 and 10"),
        x if x % 2 == 0 => println!("Number is even"),
        _ => println!("number is unknown"),

    }
}