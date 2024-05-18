mod shape;
use shape::Rectangle;

fn main() {
    let rect1 = Rectangle::new(100, 50);
    println!("Area of our rectangle {}", rect1.area());
}