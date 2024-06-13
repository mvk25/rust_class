mod assg;
use assg::*;

fn main() {
    let language = assg::Language::new("Go");
    let version = assg::Language::new(2);

    let rustaceans = assg::Language::new("Rust".to_string());
    let float_rust = assg::Language::new(4.3);
    let isrust = assg::Language::new(true);
    let rust_tuple = assg::Language::new((10, 20));
    let rust_char = assg::Language::new('a');
    let rust_array = assg::Language::new([4,5,6]);
    let rust_slice = assg::Language::new(&[12, 12, 12]);

    let a = float_rust.clone_value();
    println!("{:?}", float_rust);
    println!("{:?}", a);

}