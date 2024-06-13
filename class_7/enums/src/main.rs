enum Culture<Y>{
    ONE(Y),
    TWO(Y)
}

fn main() {
    let int = Culture::ONE(56);
    let string = Culture::TWO
    ("Hello world".to_string());

    match int {
        Culture::ONE(value) => println!("The value is {} value", value),
        Culture::TWO(value) => println!("Culture is two"),
    }
}
