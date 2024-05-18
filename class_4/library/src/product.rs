#[allow(unused_variables)]
pub struct Library {
    pub name: String,
    pub status: bool,
    pub category: String,
}

#[allow(dead_code)]
impl Library {
    pub fn new(name: String, status: bool, category: String) -> Self{
        Library {
            name,
            status,
            category
        }
    }

    pub fn avail(&self) -> bool {
        self.status
    }
}