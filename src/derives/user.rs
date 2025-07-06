use crate::util::{
    api_helper,
    paging::PageIterator,
    responses::{CountResponse, CurrencyResponse, UsernameHistoryResponse},
    ResultExtensions, RobloxResult,
};
use async_trait::async_trait;

async fn get_generic_count(user_id: i64, channel: &str, cookie: Option<&str>) -> RobloxResult<i32> {
    api_helper::get(
        format!(
            "https://friends.roblox.com/v1/users/{}/{}/count",
            user_id, channel
        ),
        cookie
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

    async fn currency(&self, cookie: Option<&str>) -> RobloxResult<i64> {
        api_helper::get(
            format!(
                "https://economy.roblox.com/v1/users/{}/currency",
                self.id()
            ),
            cookie
        )
        .await
        .map_async(api_helper::deserialize_body::<CurrencyResponse>)
        .await
        .map(|data| data.robux)
    }

    async fn has_premium(&self, cookie: Option<&str>) -> bool {
        api_helper::deserialize_body::<bool>(
            api_helper::get(
                format!(
                    "https://premiumfeatures.roblox.com/v1/users/{}/validate-membership",
                    self.id()
                ),
                cookie
            )
            .await
            .unwrap(),
        )
        .await
    }

    async fn friend_count(&self, cookie: Option<&str>) -> RobloxResult<i32> {
        get_generic_count(self.id(), "friends", cookie).await
    }

    async fn follower_count(&self, cookie: Option<&str>) -> RobloxResult<i32> {
        get_generic_count(self.id(), "followers", cookie).await
    }

    async fn following_count(&self, cookie: Option<&str>) -> RobloxResult<i32> {
        get_generic_count(self.id(), "followings", cookie).await
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
}
