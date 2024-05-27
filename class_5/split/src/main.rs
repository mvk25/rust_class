fn main() {
    let datetime = "2024-05-23T12:34:56";
    // let time = datetime.split('T').collect::<Vec<&str>>()[1];
    let time;
    if let Some(x) = index(datetime) {
        time = &datetime[x+1..];
        println!("{:?}", time);
    } else {
        eprintln!("Pattern not found");
    }
}

fn index(datetime: &str) -> Option<usize>{
    datetime.find('T')
}