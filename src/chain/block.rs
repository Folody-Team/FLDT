use std::{time::SystemTime, collections::HashMap};


#[derive(Debug,Clone)]
pub enum Block {
  Id(String),
  BlockData(BlockData)
}


pub struct ChainFLD {
  pub blocks: Vec<Vec<HashMap<String, Block>>>,
  pub(crate) provider: providerData
}

#[derive(Debug,Clone)]
pub struct providerData {
  pub name: String,
  pub shortName: String,
  pub version: String,
  pub author: String,
  pub provider: String,

}

#[derive(Debug,Clone)]
pub struct BlockData {
  pub fromAddr: String,
  pub toAddr: String,
  pub amount: f32,
  pub transactionMsg: String,
  pub createdAt: SystemTime,
  pub verifiedBy: Vec<String>,
  pub providerData: providerData

}