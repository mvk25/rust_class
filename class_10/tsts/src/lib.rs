use std::cmp::PartialOrd;

struct SuperBike{
    model: String,
    factory: bool
}

impl SuperBike{
    pub fn new(model: String, factory: bool) -> Self {
        SuperBike {
            model,
            factory
        }
    }
}

pub fn bike_count() -> usize {
    let aprilia: SuperBike = SuperBike::new("RS1000".to_string(), true);
    let pramac: SuperBike = SuperBike::new("SF1000".to_string(), false);

    let bike_vec = vec![aprilia, pramac];
    bike_vec.len()

}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn say_hello(str: String) -> String {
    format!("{}", str)
}

pub fn find_word(text: &str, word: &str) -> Option<usize> {
    text.find(word)
}

pub fn parameter_checker(par: String) -> bool {
    let v = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let contained = v.into_iter().find(|x| *x == par);
    match contained {
        Some(par) => true,
        None => false
    }
}

// Write a generic function that takes in a parameter as a list and returns the largest number
// Write a positive and negative test of the function

fn largest_number<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut big_num = list[0];
    for &i in list {
        if i > big_num {
            big_num = i;
        }
    }
    big_num
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_number() {
        let array: [i32; 6] = [10, -20, 40, 70, 30, 100];
        assert_eq!(largest_number(&array), 100);
        
        let vec_array: Vec<f64> = vec![10.1, 12.2, 23.4, 43.3, 434.6];
        assert_eq!(largest_number(&vec_array), 434.6);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_ne!(result, 6);
    }

    #[test]
    fn contains() {
        let res = say_hello("Ireland".to_string());
        assert!(res.contains("Ireland"))
    }

    #[test]
    fn bike_count_test() {
        let num: usize = bike_count();
        assert_eq!(num, 2);
    }

    #[test]
    fn find_word_test() {
        assert_eq!(find_word("The quick brown fox is here", "The"), Some(0))
    }

    #[test]
    fn word_doesnt_exist() {
        assert_eq!(find_word("The quick fox is not here", "zzz"), None)
    }

    #[test]
    fn test_parameter_checker() {
        assert_eq!(parameter_checker("one".to_string()), true)
    }

    #[test]
    fn test_parameter_checker_2() {
        let a = "five".to_string();
        assert_ne!(parameter_checker(a), true)
    }

}

