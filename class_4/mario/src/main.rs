#[derive(Debug)]
struct SuperMario<'a> {
    name: String,
    level: &'a mut u32,
    score: &'a mut u32,
}

impl <'a> SuperMario<'a> {
    fn new(name: String, level: &'a mut u32, score: &'a mut u32) -> Self {
        SuperMario {
            name,
            level,
            score
        }
    }

    fn increment_level(&mut self) {
        // let mut lev = 0;
        *self.level = *self.level + 1;
    }

    fn increment_score(&mut self, amount: u32) {
        *self.score += amount;
    }
}

fn main() {
    let mut init_level = 32;
    let mut init_score = 0;
    let mut luigi = SuperMario::new(String::from("Luigi"), &mut init_level, &mut init_score);
    luigi.increment_level();
    luigi.increment_score(100);
    println!("{:?}", luigi);
}

/// Create a program that takes in an input from the terminal and
/// a second one from the terminal , divides the first input from the second one.
/// Test for extreme conditions
/// Div by Zero
/// Integer Overflow