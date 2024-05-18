mod product;
use product::Library;

fn main() {
    let rust_book = Library::new(String::from("Rust Book"), true, String::from("Education Books"));
    println!("{} availability: {}", rust_book.name, rust_book.avail());
}