// Optimization of the Fibonacci using hashmaps

use std::collections::HashMap;

fn fastfib(num: u32, memo: &mut HashMap<u32, u32>) -> u32 {
    if let Some(&val) = memo.get(&num) {
        return val;
    }
    
    let mut n = 0;
    let result = match num {
        0 => 0,
        1 => 1,
        _ => {
            n = fastfib(num - 1, memo) + fastfib(num - 2, memo);
            n
        }
    };

    memo.insert(num, n);
    result
}

fn main() {
    let mut memo = HashMap::new();
    for i in 2..10 {
        println!("{i} {}", fastfib(i, &mut memo));
    }
}