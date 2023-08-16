pub struct ChainFLD {
  pub blocks: Vec,
pub(crate) provider: providerData,
}

pub struct providerData {
  pub name: String,
  pub shortName: String,
  pub version: String,
  pub author: String,
  pub provider: String,

}
pub struct BlockData {
  pub fromAddr: String,
  pub toAddr: String,
  pub amount: f32,
  pub transactionMsg: String,
  pub createdAt: i64,
  pub verifiedBy: Vec<String>,
  pub providerData: providerData

}
