use std::{fs::File, io::ErrorKind};
// use std::io::ErrorL

pub fn results() -> File {
    let greetings = File::open("hello.txt");

    let greeting_file = match greetings {
        Ok(file) => file,
        Err(err) => match err.kind() { 
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(err) => panic!("Problem creating file"),
            },

            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };

    // println!("{:?}", greeting_file);
    greeting_file
}