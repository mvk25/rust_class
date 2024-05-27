use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut vec: Vec<u32>= Vec::new();

    for item in 1..=10 {
        let num = rng.gen_range(1..10);
        println!("{}", num);
        vec.push(num);
    }

    vec.sort();
    println!("{:?}", vec);


}