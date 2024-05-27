use std::collections::HashMap;

fn main() {

    let mut maps: HashMap<String, i32> = HashMap::new();
    
    maps.insert("Kevin".to_string(), 22);
    maps.insert("Michael".to_string(), 32);
    maps.insert("Oscar".to_string(), 42);
    
    println!("{:?}", maps);

    for key in maps.values() {
        println!("{}", key);
    }

    maps.entry("Michael".to_string()).or_insert(45);

    println!("{:?}", maps);

}