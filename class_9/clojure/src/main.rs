fn create_multiples(factor: i32) -> impl Fn(i32) -> i32 {
    move |i: i32| {
        i * factor
    }
}

fn create_multiples_2() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x: i32| x + 1)
}

fn main() {
    let impl_1 = create_multiples(10);
    println!("{}", impl_1(3));
    let x = create_multiples_2();
    println!("{}", x(10));
}
//A list of functions, 
//Functions that take in predicates