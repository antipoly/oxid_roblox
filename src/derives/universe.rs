use async_trait::async_trait;

use crate::{
  models::{Badge, GamePass, SocialLink, UniverseLiveStats},
  util::{
    api_helper,
    paging::{identity_mapper, PageIterator},
    responses::{ApiArrayResponse, UniverseFavoriteCountResponse},
    ResultExtensions, RobloxResult,
  },
};

#[async_trait]
pub trait Universe {
  #[doc(hidden)]
  fn id(&self) -> i64;

  async fn favorite_count(&self, cookie: Option<String>) -> RobloxResult<i64> {
    api_helper::get(format!("https://games.roblox.com/v1/games/{}/favorites/count", self.id()), cookie)
      .await
      .map_async(api_helper::deserialize_body::<UniverseFavoriteCountResponse>)
      .await?
      .map(|data| data.favorites_count)
  }

  fn badges(&self, cookie: Option<String>) -> PageIterator<Badge, Badge> {
    PageIterator::new(
      format!("https://badges.roblox.com/v1/universes/{}/badges", self.id()),
      identity_mapper,
      cookie,
    )
  }

  async fn live_stats(&self, cookie: Option<String>) -> RobloxResult<UniverseLiveStats> {
    api_helper::get(format!("https://develop.roblox.com/v1/universes/{}/live-stats", self.id()), cookie)
      .await
      .map_async(api_helper::deserialize_body)
      .await?
  }

  fn gamepasses(&self, cookie: Option<String>) -> PageIterator<GamePass, GamePass> {
    PageIterator::new(
      format!("https://games.roblox.com/v1/games/{}/game-passes", self.id()),
      identity_mapper,
      cookie,
    )
  }

  async fn social_links(&self, cookie: Option<String>) -> RobloxResult<Vec<SocialLink>> {
    api_helper::get(format!("https://games.roblox.com/v1/games/{}/social-links/list", self.id()), cookie)
      .await
      .map_async(api_helper::deserialize_body::<ApiArrayResponse<SocialLink>>)
      .await?
      .map(|data| data.data)
  }
}
