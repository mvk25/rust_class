pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for i in list.iter() {
        if *i > largest {
            largest = *i;
        }
    }
    largest
}

pub fn largest_integer(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for num in list.iter() {
        if *num > largest {
            largest = *num;
        }
    }
    largest
}

fn main() {
    let number = [1, 20, 3, 100];
    let large_number = largest(&number);
    println!("{}", large_number);


    let characters = ['a', 'x', 'f', 'z'];
    let large_char = largest(&characters);
    println!("{}", large_char);
}

