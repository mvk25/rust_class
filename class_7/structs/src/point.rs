struct Point<D> {
    x: D,
    y: D
}

impl <D> Point<D> {
    fn new(x: D, y: D) -> Self {
        Point {
            x,
            y
        }
    }

    fn get_x(&self) -> &D {
        &self.x
    }
}