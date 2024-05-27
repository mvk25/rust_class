fn sum(vec: Vec<(i32, i32)>) -> i32 {
    let mut sum = 0;
    if vec.iter().all(|x| x.1 % 2 == 0) {
        sum = vec.iter().fold(0, |acc, elem| elem.0 + acc);
    } else if vec.iter().all(|x| x.0 % 2 == 1) {
        sum = vec.iter().fold(0, |acc, elem| elem.1 + acc);
    }

    sum
}

fn main() {
    let vec1: Vec<(i32, i32)> = vec![(10, 20), (20, 10), (30, 40)];
    let vec2: Vec<(i32, i32)> = vec![(11, 20), (21, 10), (31, 40)];

    println!("{}", sum(vec1));
    println!("{}", sum(vec2));
}
