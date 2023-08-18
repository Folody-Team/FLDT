use std::{time::SystemTime, collections::HashMap};


#[derive(Debug,Clone)]
pub enum Block {
  Id(String),
  BlockData(BlockData)
}

#[derive(Debug,Clone)]
pub struct ChainFLD {
  pub blocks: Vec<Vec<HashMap<String, Block>>>,
  pub(crate) provider: ProviderData
}

#[derive(Debug,Clone)]
pub struct ProviderData {
  pub name: String,
  pub short_name: String,
  pub version: String,
  pub author: String,
  pub provider: String,

}

#[derive(Debug,Clone)]
pub struct BlockData {
  pub from_addr: String,
  pub to_addr: String,
  pub amount: f32,
  pub transaction_msg: String,
  pub created_at: SystemTime,
  pub verified_by: Vec<String>,
  pub provider_data: ProviderData

}