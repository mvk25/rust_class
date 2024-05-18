//fibonaccu
fn main() {
    println!("{}", fibonacci(10));
    println!("{}", fibonacci(9));
    println!("{}", fibonacci(8));
    println!("{}", fibonacci(2));

}

fn fibonacci(num: u32) -> u32 {
    if num == 0 {
        0
    } else if num == 1 {
        1
    } else {
        fibonacci(num - 1) + fibonacci(num - 2)
    }
}
