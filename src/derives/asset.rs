use async_trait::async_trait;

use crate::{
  models::AssetResaleData,
  util::{api_helper, ResultExtensions, RobloxResult},
};

#[async_trait]
pub trait Asset: Send {
  #[doc(hidden)]
  fn id(&self) -> i64;
  async fn resale_data(&self, cookie: Option<String>) -> RobloxResult<AssetResaleData> {
    api_helper::get(format!("https://economy.roblox.com/v1/assets/{}/resale-data", self.id()), cookie)
      .await
      .map_async(api_helper::deserialize_body)
      .await?
  }
}
