struct Circle {
    radius: f64,
}

struct Rectangle {
    length: f64,
    width: f64
}

trait Area {
    fn area(&self) -> f64;
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }
}

fn main() {
    let rect = Rectangle {
        length: 20.0,
        width: 10.0
    };

    let circle = Circle {
        radius: 14.0
    };

    println!("Area of rectangle: {}", rect.area());
    println!("Area of circle: {}", circle.area());

}