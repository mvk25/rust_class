pub struct Rectangle {
    pub length: u32,
    pub width: u32,
}

impl Rectangle {
    pub fn new(length: u32, width: u32) -> Self {
        Rectangle {
            length,
            width
        }
    }

    pub fn area(&self) -> u32 {
        self.length * self.width
    }
}


// Assignment
// Research on the document attribute
// Previos assignment should contain documentation
