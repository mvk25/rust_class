// ## Create a struct with vectors - String, str, Vec, HashMaps
// ## Call a function of the struct to update a vector, hashmap.
use std::fmt;

// #[derive(Debug)]
struct MyVector {
    vector: Vec<u32>,
    length: usize
}

impl MyVector {
    fn new(len: usize) -> Self {
        let new_vec = Vec::with_capacity(len);
        MyVector {
            vector: new_vec,
            length: len,
        }
    }

    fn push(&mut self, int: u32) {
        self.vector.push(int);
    }
}

impl fmt::Debug for MyVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.vector)
    }
}

fn main() {
    let mut new_vec = MyVector::new(10);
    println!("{:?}", new_vec);

    new_vec.push(10);
    new_vec.push(20);
    new_vec.push(40);
    new_vec.push(50);
    new_vec.push(60);
    new_vec.push(70);
    new_vec.push(80);
    new_vec.push(90);
    new_vec.push(100);
    // new_vec.push(110);

    println!("{:?}", new_vec);
}