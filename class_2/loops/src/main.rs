fn main() {
    // print out an array or items in reverse

    // let array: &[i32] = &[1, 2, 3, 4, 5, 6];
    // let iter: Vec<&i32> = array.into_iter().rev().collect();

    // for i in iter {
    //     println!("{}", i);
    // }

    //print 50 to 1 using a range operate
    for i in (1..50).rev() {
        println!("{i}");
    }
}
