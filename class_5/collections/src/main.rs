// fn main() {
//     let mut x = Vec::new();
//     x.push(1);
//     x.push(2);
//     x.push(3);

//     for item in x.iter_mut() {
//         *item = *item + 1;
//     }

//     println!("{:?}", x);
// }


fn main() {
    let mut x = Vec::new();

    x.push("Hello");
    x.push("World");
    x.push(".....");

    let finals = x.len().saturating_sub(0);
    println!("{}", finals);
    x.truncate(finals);

    println!("after {:?}", x);
}