use std::collections::HashMap;

fn main() {
    println!("{:?}", character_count("The lazy fox jumped over the bridge lazy"));

}

// fn word_count(text: &str) -> HashMap<&str, u32> {
//     let word_chart = HashMap::new();
//     let word_vec: Vec<&str> = text.split(' ').collect();

//     for word in word_vec.iter() {
//         word_chart.entry(word).or_insert(0);
//     }

//     for word in word_vec.iter() {
//         *word_chart.get_mut(word).unwrap() += 1;
//     }

//     word_chart
// }

fn character_count(text: &str) -> HashMap<char, u32> {
    let mut alphabet_chart = HashMap::new();

    for char in text.chars() {
        alphabet_chart.entry(char).or_insert(0);
    }
    
    for char in text.chars() {
        *alphabet_chart.get_mut(&char).unwrap() += 1;
    }

    alphabet_chart
} 
