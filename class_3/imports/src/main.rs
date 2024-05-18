#[allow(unused_variables)]
struct User {
    name: String,
    age: i32,
    id_number: i32,
}

#[allow(dead_code)]
impl User {
    fn new(name: String, age: i32, id_number: i32) -> Self{
        User {
            name,
            age, 
            id_number
        }
    }
    
    fn walk(&self) {
        println!("User {} is walking", &self.name);
    }

    pub fn eat(&self) {
        println!("User {} is eating", &self.name);
    }

    pub fn is_adult(&self) -> bool{
        if self.age > 18 {
            return true;
        }

        return false;
    }
}

fn main() {
    let barbie = User{name: String::from("Barbie"), age: 100, id_number: 24324};
    let result = barbie.is_adult();
    println!("Is adult? {}", result);
}