pub fn get_value_by_index(index: usize) {
    let v: Vec<u32> = vec![1,2,3,4,5,6,7,8,9,10];
    match v.get(index) {
        Some(n) => println!("{}", n),
        None => panic!("Out of index range"),
    }
}
