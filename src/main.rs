mod math;
mod chain;

use math::{sha256 as hash};

fn main() {
    let res = hash("hello".to_string());
    println!("Result: {}", res);
}
