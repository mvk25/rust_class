fn multiply_by_three<F>(par_1: F) -> i32 where F: Fn(i32) -> i32 {
    par_1(10)
}

//let multiple = |x: i32| -> i32 {
    //    x * 3
    //};

    //let result: i32 = multiply_by_three(multiple);
    //println!("Result is {}", result);
