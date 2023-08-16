mod block;

use rand::{distributions::Alphanumeric, Rng};
use crate::math::{sha256 as hash};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use block::{BlockData as Data};
use block::{ChainFLD as FLD};
use block::{providerData as Provider};

pub impl FLD {
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

  fn hash2binary(hash: &[u8]) {

  }

  pub fn initialization(&mut self, fromAddr: String, toAddr: String) {
    let mut block: [Any; 2] = [
      hash(
        rand::thread_rng()
          .sample_iter(&Alphanumeric)
          .take(7)
          .map(char::from)
          .collect()
      )
    ];
    let mut blockInside = HashMap::new();
    let blockData = Data {
      fromAddr: fromAddr,
      toAddr: toAddr,
      amount: 0.0,
      transactionMsg: "".to_owned(),
      createdAt: SystemTime::now(),
      verifiedBy: [],
      providerData: self.provider
    };

    blockInside.insert(block[0].to_string().to_owned(), hash(
      rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
    ));
    blockInside.insert("blockData".to_owned(), blockData);
    block[1] = blockData;
    self.blocks.push(block);
  }
  
  pub fn isValid(&self, block: &[Any; 2], previous_block: &[Any; 2]) {

  }
  pub fn tryAdd (&mut self, block: [Any; 2]) {
    let latest = self.blocks.last().expect("there is at least one block");
    if self.isValid(&block, latest) {
        self.blocks.push(block);
    } else {
        error!("could not add block");
    }
  }
}