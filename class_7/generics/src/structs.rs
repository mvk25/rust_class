pub struct Point<T> {
    pub x: D,
    pub y: D
}

impl <D> Point<D> {
    pub fn new(x: D, y: D) -> Self {
        Point {
            x,
            y
        }
    }

    pub fn get_x(&self) -> &D {
        &self
    }
}