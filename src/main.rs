mod math;
mod chain;

use chain::{FLD as FLD};
use chain::{ChainFLD as ChainFLD};
use math::{sha256 as hash};

fn main() {
    let blockChain = ChainFLD::new();
    println!("{:?}", blockChain);
}
