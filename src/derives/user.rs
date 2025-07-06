use crate::util::{
    api_helper,
    paging::PageIterator,
    responses::{CountResponse, CurrencyResponse, UserGroupRolesResponse, UsernameHistoryResponse},
    ResultExtensions, RobloxResult,
};
use async_trait::async_trait;

async fn get_generic_count(user_id: i64, channel: &str) -> RobloxResult<i32> {
    api_helper::get(
        format!(
            "https://friends.roblox.com/v1/users/{}/{}/count",
            user_id, channel
        ),
        None
    )
    .await
    .map_async(api_helper::deserialize_body::<CountResponse>)
    .await
    .map(|data| data.count)
}

#[async_trait]
pub trait User {
    #[doc(hidden)]
    fn id(&self) -> i64;

    async fn currency(&self) -> RobloxResult<i64> {
        api_helper::get(
            format!(
                "https://economy.roblox.com/v1/users/{}/currency",
                self.id()
            ),
            None
        )
        .await
        .map_async(api_helper::deserialize_body::<CurrencyResponse>)
        .await
        .map(|data| data.robux)
    }

    async fn has_premium(&self) -> bool {
        api_helper::deserialize_body::<bool>(
            api_helper::get(
                format!(
                    "https://premiumfeatures.roblox.com/v1/users/{}/validate-membership",
                    self.id()
                ),
                None
            )
            .await
            .unwrap(),
        )
        .await
    }

    async fn friend_count(&self) -> RobloxResult<i32> {
        get_generic_count(self.id(), "friends").await
    }

    async fn follower_count(&self) -> RobloxResult<i32> {
        get_generic_count(self.id(), "followers").await
    }

    async fn following_count(&self) -> RobloxResult<i32> {
        get_generic_count(self.id(), "followings").await
    }

    fn username_history(&self) -> PageIterator<UsernameHistoryResponse, String> {
        PageIterator::new(
            format!(
                "https://users.roblox.com/v1/users/{}/username-history",
                self.id()
            ),
            |data| data.name.clone(),
            None
        )
    }

    async fn group_roles(&self) -> RobloxResult<Vec<UserGroupRolesResponse>> {
      api_helper::get(
        format!(
          "https://groups.roblox.com/v2/users/{}/groups/roles",
          self.id()
        ),
        None
      ).await
      .map_async(api_helper::deserialize_body::<Vec<UserGroupRolesResponse>>)
      .await
    }
}
