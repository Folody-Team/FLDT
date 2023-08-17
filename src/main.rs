mod math;
mod chain;

use chain::{FLD as FLD};
use chain::{ChainFLD as ChainFLD};

fn main() {
    let mut block_chain = ChainFLD::new();
    block_chain.initialization(String::from("hello"), String::from("hello"));
    block_chain.initialization(String::from("siuu"), String::from("lolllll"));
    println!("{:#?}", std::mem::size_of_val(&*block_chain.blocks));
}
