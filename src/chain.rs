mod block;

use any_vec::any_value::Unknown;
use rand::{distributions::Alphanumeric, Rng};
use crate::math::{sha256 as hash};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use block::{Block as Block};
use block::{BlockData as Data};
pub use block::{ChainFLD as ChainFLD};
use block::{providerData as Provider};

pub trait FLD {
  fn new() -> Self;
  fn initialization(&mut self, fromAddr: String, toAddr: String);
}

impl FLD for ChainFLD {
  fn new() -> Self {
    Self { 
      blocks: vec![],
      provider:  Provider {
        name: "Folody Crypto Wallet".to_owned(),
        shortName: "FLCWL".to_owned(),
        version: "V1.0.2RTM".to_owned(),
        author: "Folody Crypto".to_owned(),
        provider: "Discord".to_owned(),
      }
    }
  }


  fn initialization(&mut self, fromAddr: String, toAddr: String) {
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
    let mut blockInside: HashMap<String, Block> = HashMap::new();
    let blockData = Data {
      fromAddr: fromAddr,
      toAddr: toAddr,
      amount: 0.0,
      transactionMsg: "".to_owned(),
      createdAt: SystemTime::now(),
      verifiedBy: [].to_vec(),
      providerData: self.provider.clone()
    };

    match id.clone().get(&String::from("id")).unwrap() {
      Block::Id(value) => {
        blockInside.insert( String::from(&value.to_string()).to_owned(), Block::Id(hash(
          rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect()
        )));
      },
        Block::BlockData(_) => todo!(),
    }
    
    blockInside.insert(String::from("blockData"), Block::BlockData(blockData));
    
    block.push(blockInside);
    self.blocks.push(block);
  }
  

}