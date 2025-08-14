use crate::{
  models::{GroupRole, GroupSettings, GroupShout, JoinRequest, Member, SocialLink, WallPost},
  util::{
    api_helper,
    paging::{identity_mapper, PageIterator},
    responses::{ApiArrayResponse, GroupRolesResponse},
    ResultExtensions, RobloxResult,
  },
};
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
pub trait Group {
  #[doc(hidden)]
  fn id(&self) -> i64;

  async fn update_shout(&mut self, message: String, cookie: Option<&str>) -> RobloxResult<GroupShout> {
    api_helper::patch(
      format!("https://groups.roblox.com/v1/groups/{}/status", self.id()),
      json!({ "message": message }),
      cookie,
    )
    .await
    .map_async(api_helper::deserialize_body)
    .await?
  }

  async fn accept_join_request(&self, user_id: i64, cookie: Option<&str>) -> RobloxResult<()> {
    api_helper::post(
      format!("https://groups.roblox.com/v1/groups/{}/join-requests/users/#{}", self.id(), user_id),
      json!({}),
      cookie,
    )
    .await
    .map(|_| ())
  }

  async fn decline_join_request(&self, user_id: i64, cookie: Option<&str>) -> RobloxResult<()> {
    api_helper::delete(
      format!("https://groups.roblox.com/v1/groups/{}/join-requests/users/{}", self.id(), user_id),
      cookie,
    )
    .await
    .map(|_| ())
  }

  async fn kick(&self, user_id: i64, cookie: Option<&str>) -> RobloxResult<()> {
    api_helper::delete(format!("https://groups.roblox.com/v1/groups/{}/users/{}", self.id(), user_id), cookie)
      .await
      .map(|_| ())
  }

  async fn roles(&self, cookie: Option<&str>) -> RobloxResult<Vec<GroupRole>> {
    api_helper::get(format!("https://groups.roblox.com/v1/groups/{}/roles", self.id()), cookie)
      .await
      .map_async(api_helper::deserialize_body::<GroupRolesResponse>)
      .await?
      .map(|data| data.roles)
  }

  async fn set_user_role(&self, user_id: i64, role_id: i64, cookie: Option<&str>) -> RobloxResult<()> {
    api_helper::patch(
      format!("https://groups.roblox.com/v1/groups/{}/users/{}", self.id(), user_id),
      json!({ "roleId": role_id }),
      cookie,
    )
    .await
    .map(|_| ())
  }

  fn members(&self, cookie: Option<&str>) -> PageIterator<Member, Member> {
    PageIterator::new(
      format!("https://groups.roblox.com/v1/groups/{}/users", self.id()),
      identity_mapper,
      cookie,
    )
  }

  async fn settings(&self, cookie: Option<&str>) -> RobloxResult<GroupSettings> {
    api_helper::get(format!("https://groups.roblox.com/v1/groups/{}/settings", self.id()), cookie)
      .await
      .map_async(api_helper::deserialize_body)
      .await?
  }

  async fn update_settings(
    &self,
    is_approval_required: Option<bool>,
    are_enemies_allowed: Option<bool>,
    are_group_funds_visible: Option<bool>,
    are_group_games_visible: Option<bool>,
    cookie: Option<&str>,
  ) -> RobloxResult<()> {
    api_helper::patch(
      format!("https://groups.roblox.com/v1/groups/{}/settings", self.id()),
      json!({
          "isApprovalRequired": is_approval_required,
          "areEnemiesAllowed": are_enemies_allowed,
          "areGroupFundsVisible": are_group_funds_visible,
          "areGroupGamesVisible": are_group_games_visible
      }),
      cookie,
    )
    .await
    .map(|_| ())
  }

  async fn delete_all_wall_posts_from_user(&self, user_id: i64, cookie: Option<&str>) -> RobloxResult<()> {
    api_helper::delete(
      format!("https://groups.roblox.com/v1/groups/{}/wall/users/{}/posts", self.id(), user_id),
      cookie,
    )
    .await
    .map(|_| ())
  }

  async fn delete_wall_post(&self, wall_post_id: i64, cookie: Option<&str>) -> RobloxResult<()> {
    api_helper::delete(
      format!("https://groups.roblox.com/v1/groups/{}/wall/posts/{}", self.id(), wall_post_id),
      cookie,
    )
    .await
    .map(|_| ())
  }

  fn wall_posts(&self, cookie: Option<&str>) -> PageIterator<WallPost, WallPost> {
    PageIterator::new(
      format!("https://groups.roblox.com/v2/groups/{}/wall/posts", self.id()),
      identity_mapper,
      cookie,
    )
  }

  fn join_requests(&self, cookie: Option<&str>) -> PageIterator<JoinRequest, JoinRequest> {
    PageIterator::new(
      format!("https://groups.roblox.com/v1/groups/{}/join-requests", self.id()),
      identity_mapper,
      cookie,
    )
  }

  async fn social_links(&self, cookie: Option<&str>) -> RobloxResult<Vec<SocialLink>> {
    api_helper::get(format!("https://groups.roblox.com/v1/groups/{}/social-links", self.id()), cookie)
      .await
      .map_async(api_helper::deserialize_body::<ApiArrayResponse<SocialLink>>)
      .await?
      .map(|data| data.data)
  }

  async fn join_request_from_user(&self, user_id: i64, cookie: Option<&str>) -> RobloxResult<Option<JoinRequest>> {
    api_helper::get(
      format!("https://groups.roblox.com/v1/groups/{}/join-requests/users/{}", self.id(), user_id),
      cookie,
    )
    .await
    .map_async(api_helper::deserialize_body)
    .await?
  }
}