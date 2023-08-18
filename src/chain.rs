mod block;


use rand::{distributions::Alphanumeric, Rng};
use crate::math::{sha256 as hash};
use std::collections::HashMap;
use std::time::{SystemTime};
use block::{Block as Block};
use block::{BlockData as Data};
pub use block::{ChainFLD as ChainFLD};
use block::{ProviderData as Provider};

pub trait FLD {
  fn new() -> Self;
  fn initialization(&mut self, from_addr: String,to_addrr: String);
}

impl FLD for ChainFLD {
  fn new() -> Self {
    Self { 
      blocks: vec![],
      provider:  Provider {
        name: "Folody Crypto Wallet".to_owned(),
        short_name: "FLCWL".to_owned(),
        version: "V1.0.2RTM".to_owned(),
        author: "Folody Crypto".to_owned(),
        provider: "Folody".to_owned(),
      }
    }
  }


  fn initialization(&mut self, from_addr: String, to_addr: String) {
    let mut block: Vec<HashMap<String, Block>> = Vec::new();
    let mut id: HashMap<String, Block> = HashMap::new();
    id.insert(String::from("id"), Block::Id(hash(
      rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
    )));
    block.push(id.clone());
    let mut block_inside: HashMap<String, Block> = HashMap::new();
    let block_data = Data {
      from_addr: from_addr.to_string(),
      to_addr: to_addr.to_string(),
      amount: 0.0,
      transaction_msg: "".to_owned(),
      created_at: SystemTime::now(),
      verified_by: [].to_vec(),
      provider_data: self.provider.clone()
    };

    match id.clone().get(&String::from("id")).unwrap() {
      Block::Id(value) => {
        block_inside.insert( String::from(&value.to_string()).to_owned(), Block::Id(hash(
          rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect()
        )));
      },
        Block::BlockData(_) => todo!(),
    }
    
    block_inside.insert(String::from("blockData"), Block::BlockData(block_data));
    
    block.push(block_inside);
    self.blocks.push(block);
  }
  

}