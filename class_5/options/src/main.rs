mod results;

use std::fs::{self, read_to_string, File};


fn main() {
 
    let our_file = File::open("hello.txt");

    let mut username_file = match our_file {
        Ok(file) => file,
        Err(err) => Err(err),
    };
    let emptyhad = String::new();
    let contents = fs::read_to_string(emptyhad);

    match contents {
        Ok(_) => Ok(emptyhad),
        Err(e) => Err(e),
    };
} 