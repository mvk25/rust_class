
#[derive(Debug, Clone)]
pub struct Language<T: Clone> {
    pub m: T
}

impl<T: Clone> Language<T> {
    pub fn new(text: T) -> Self {
        Language {
            m: text
        }
    }

    pub fn clone_value(&self) -> T {
        self.m.clone()
    }
}

