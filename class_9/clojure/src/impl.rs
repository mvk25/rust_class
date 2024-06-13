fn create_multiples(factor: i32) -> impl Fn(i32) -> i32 {
    move |i: i32| {
        i * factor
    }
}

fn create_multiples_2(factor: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x: i32| x + 1)
}

// let double = create_multiples(3);
//     println!("doubles {}", double(40));

// fn main() {
//     let nums: Vec<u32> = vec![1,2,3,4,5,6,7,8,9,10];

//     // let evens: Vec<&u32> = nums.iter().filter(|x| *x % 2 == 0).collect();

//     // println!("{:?}", evens);
//     let five = nums.iter().find(|x| **x == 5).unwrap();
//     println!("{}", five);
// }

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u32,
// }

// impl Person {
//     fn new(name: String, age: u32) -> Self {
//         Person {
//             name,
//             age
//         }
//     }
// }

// fn main() {
//     let person_one: Person = Person::new("Lebron".to_string(), 40);
//     let person_two: Person = Person::new("Irving".to_string(), 30);

//     let people: Vec<Person> = vec![person_one, person_two];

//     let third_floor: Person = people.into_iter().find(|person: &Person| person.age > 30).unwrap();
//     println!("{:?}", third_floor);
// }