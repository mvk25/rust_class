#[derive(Debug)]
pub struct Vehicle<T,X> {
    pub cus_one: T,
    pub cus_two: X
}

impl<T, X> Vehicle<T, X> {
    pub fn new(one:T, two: X) -> Self {
        Vehicle {
            cus_one: one,
            cus_two: two
        }
    }

    pub fn swap(&mut self, other_client:Vehicle<T, X>) {
        self.cus_one = other_client.cus_one    }
}