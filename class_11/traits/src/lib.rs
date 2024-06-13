use std::fmt::Display;
use std::ops::Div;
use num::Zero;

struct Rect {
    length: u32,
    width: u32,
}   

impl Rect {
    fn new(length: u32, width: u32) -> Self {
        Rect {
            length,
            width
        }
    }
}
trait Area {
    fn area(&self) -> u32;
}
impl Area for Rect {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}


pub fn divide<T>(a: T, b: T) -> Result<T, String>
where
    T: Zero + Div<Output = T> + Display + Copy + PartialOrd,
{
    if b == T::zero() {
        Err(format!("Zero Division Error"))
    } else {
        Ok(a / b)
    }
}

// pub fn divide(a: f64, b: f64) -> Result<f64, String> {
//     if b == 0.0 {
//         Err("Zero Division Error".to_string())
//     }
//     else {
//         Ok(a / b)
//     }
// }

fn vowel_or_cons(tuple: (char, i32)) -> String {
    match tuple.0 {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("Vowel"),
        _ => format!("Consonant")
    }

}

pub fn smaller_val<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { return b; }
    a
}

pub fn boolean_checker(vec_bools: Vec<bool>, item_type: bool) -> String {
    let var_bool = vec_bools.into_iter().all(|elem| elem == item_type);
    match var_bool {
        true => {
            if item_type == true { 
                return format!("All trues!"); 
            }
            format!("All false!")
        },
        _ => format!("Mixed!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_checker() {
        assert_eq!(boolean_checker(vec![true, true, true], true), format!("All trues!"));
        assert_eq!(boolean_checker(vec![false, false, false], false), format!("All false!"));
        assert_eq!(boolean_checker(vec![false, true, false], true), format!("Mixed!"));
    }


    #[test]
    fn test_smaller_val() {
        assert_eq!(smaller_val(10, 20), 10);
        assert_ne!(smaller_val(10, 9), 10);
    }
    #[test]
    fn test_vowel_or_cons() {
        let our_tuple1 = ('a', 10);
        let our_tuple2: (char, i32) = ('b', 11);

        assert_eq!(vowel_or_cons(our_tuple1), format!("Vowel"));
        assert_eq!(vowel_or_cons(our_tuple2), format!("Consonant"));
    }
    #[test]
    fn test_rect_area() {
        let rect_1 = Rect::new(20, 10);
        let rect_area = rect_1.area();
        assert_eq!(rect_area, 200);

        let rect_2 = Rect::new(14, 7);
        let rect_area_2 = rect_2.area();
        assert_ne!(rect_area_2, 1);
    }

    #[test]
    fn test_closure() {
        let x = |y| y + 2;
        let ans = x(10);

        assert_eq!(ans, 12);
    }

    #[test]
    fn test_divide() {
        let result: Result<f64, String> = divide(4.0, 2.0);
        assert_eq!(result.unwrap(), 2.0);

        let res_string: Result<f64, String> = divide(3.0, 0.0);
        assert_eq!(res_string, Err("Zero Division Error".to_string()));
    }
}

