trait Describe {
    fn describe_user(&self) -> String;
}

struct Person {
    name: String,
    age: i32,
}

impl Describe for Person {
    fn describe_user(&self) -> String {
        format!("User name: {} Age {}", self.name, self.age)
    }
}

fn traut_impl() {
    let a = Person {
        name: "Kevin".to_string(),
        age: 50
    };

    println!("{}", a.describe_user());
}