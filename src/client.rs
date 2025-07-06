use crate::{
    bases::{BaseAsset, BaseGroup, BasePlugin, BaseUniverse, BaseUser},
    models::{
        Badge, EconomyAsset, Group, Place, Plugin, Presence, PreviousUsernamesSkinnyUser,
        SkinnyUser, Universe, User,
    },
    util::{
        api_helper,
        paging::{identity_mapper, PageIterator},
        responses::{ApiArrayResponse, UserPresencesResponse},
        ResultExtensions, RobloxResult,
    },
};
use serde_json::json;

// Transforms a list of ids into a comma-separated string "1,2,3,4,5" for use in some multi-get endpoints
fn ids_to_string(ids: Vec<i64>) -> String {
    let mut s = ids
        .iter()
        .map(|id| format!("{},", id.to_string()))
        .collect::<String>();
    s.pop();
    s
}

pub fn set_roblosecurity(roblosecurity: &str) {
    api_helper::set_roblosecurity(roblosecurity);
}

pub fn search_users(
    keyword: &str,
) -> PageIterator<PreviousUsernamesSkinnyUser, PreviousUsernamesSkinnyUser> {
    PageIterator::new(
        format!(
            "https://users.roblox.com/v1/users/search?keyword={}",
            keyword
        ),
        identity_mapper,
        None
    )
}

pub fn base_plugin(plugin_id: i64) -> BasePlugin {
    BasePlugin { id: plugin_id }
}

pub fn base_group(group_id: i64) -> BaseGroup {
    BaseGroup { id: group_id }
}

pub fn base_user(user_id: i64) -> BaseUser {
    BaseUser { id: user_id }
}

pub fn base_universe(universe_id: i64) -> BaseUniverse {
    BaseUniverse { id: universe_id }
}

pub fn base_asset(asset_id: i64) -> BaseAsset {
    BaseAsset { id: asset_id }
}

pub async fn authenticated_user(cookie: Option<&str>) -> SkinnyUser {
    api_helper::deserialize_body(
        api_helper::get("https://users.roblox.com/v1/users/authenticated".to_owned(), cookie)
            .await
            .unwrap(),
    )
    .await
}

pub async fn users_from_ids(
    user_ids: Vec<i64>,
    exclude_banned_users: bool,
    cookie: Option<&str>,
) -> RobloxResult<Vec<SkinnyUser>> {
    api_helper::post(
        "https://users.roblox.com/v1/users".to_owned(),
        json!({
            "userIds": user_ids,
            "excludeBannedUsers": exclude_banned_users
        }),
        cookie
    )
    .await
    .map_async(api_helper::deserialize_body::<ApiArrayResponse<SkinnyUser>>)
    .await
    .map(|data| data.data)
}

pub async fn users_from_usernames(
    usernames: Vec<&str>,
    exclude_banned_users: bool,
    cookie: Option<&str>,
) -> RobloxResult<Vec<SkinnyUser>> {
    api_helper::post(
        "https://users.roblox.com/v1/usernames/users".to_owned(),
        json!({
            "usernames": usernames,
            "excludeBannedUsers": exclude_banned_users
        }),
        cookie
    )
    .await
    .map_async(api_helper::deserialize_body::<ApiArrayResponse<SkinnyUser>>)
    .await
    .map(|data| data.data)
}

pub async fn user_presences_from_ids(user_ids: Vec<i64>, cookie: Option<&str>) -> RobloxResult<Vec<Presence>> {
    api_helper::post(
        "https://presence.roblox.com/v1/presence/users".to_owned(),
        json!({
            "userIds": user_ids
        }),
        cookie
    )
    .await
    .map_async(api_helper::deserialize_body::<UserPresencesResponse>)
    .await
    .map(|data| data.user_presences)
}

pub async fn universes_from_ids(universe_ids: Vec<i64>, cookie: Option<&str>) -> RobloxResult<Vec<Universe>> {
    api_helper::get(
        format!(
            "https://games.roblox.com/v1/games?universeIds={}",
            ids_to_string(universe_ids)
        ),
        cookie
    )
    .await
    .map_async(api_helper::deserialize_body::<ApiArrayResponse<Universe>>)
    .await
    .map(|data| data.data)
}

pub async fn places_from_ids(place_ids: Vec<i64>, cookie: Option<&str>) -> RobloxResult<Vec<Place>> {
    api_helper::get(
        format!(
            "https://games.roblox.com/v1/games/multiget-place-details?placeIds={}",
            ids_to_string(place_ids)
        ),
        cookie
    )
    .await
    .map_async(api_helper::deserialize_body::<Vec<Place>>)
    .await
}

pub async fn plugins_from_ids(plugin_ids: Vec<i64>, cookie: Option<&str>) -> RobloxResult<Vec<Plugin>> {
    api_helper::get(
        format!(
            "https://develop.roblox.com/v1/plugins?pluginIds={}",
            ids_to_string(plugin_ids)
        ),
        cookie
    )
    .await
    .map_async(api_helper::deserialize_body::<ApiArrayResponse<Plugin>>)
    .await
    .map(|data| data.data)
}

pub async fn user_from_id(user_id: i64, cookie: Option<&str>) -> RobloxResult<User> {
    api_helper::get(
        format!("https://users.roblox.com/v1/users/{}", user_id),
        cookie
    )
        .await
        .map_async(api_helper::deserialize_body)
        .await
}

pub async fn user_from_username(username: &str, cookie: Option<&str>) -> RobloxResult<Option<SkinnyUser>> {
    users_from_usernames(vec![username], false, cookie)
        .await
        .map(|users| users.first().cloned())
}

pub async fn group_from_id(group_id: i64, cookie: Option<&str>) -> RobloxResult<Group> {
    api_helper::get(
        format!("https://groups.roblox.com/v1/groups/{}", group_id),
        cookie
    )
        .await
        .map_async(api_helper::deserialize_body)
        .await
}

pub async fn user_presence_from_id(user_id: i64, cookie: Option<&str>) -> RobloxResult<Option<Presence>> {
    user_presences_from_ids(vec![user_id], cookie)
        .await
        .map(|presences| presences.first().cloned())
}

pub async fn universe_from_id(universe_id: i64, cookie: Option<&str>) -> RobloxResult<Option<Universe>> {
    universes_from_ids(vec![universe_id], cookie)
        .await
        .map(|universes| universes.first().cloned())
}

pub async fn place_from_id(place_id: i64, cookie: Option<&str>) -> RobloxResult<Option<Place>> {
    places_from_ids(vec![place_id], cookie)
        .await
        .map(|places| places.first().cloned())
}

pub async fn asset_from_id(asset_id: i64, cookie: Option<&str>) -> RobloxResult<EconomyAsset> {
    api_helper::get(
        format!(
            "https://economy.roblox.com/v2/assets/{}/details",
            asset_id
        ),
        cookie
    )
    .await
    .map_async(api_helper::deserialize_body)
    .await
}

pub async fn plugin_from_id(plugin_id: i64, cookie: Option<&str>) -> RobloxResult<Option<Plugin>> {
    plugins_from_ids(vec![plugin_id], cookie)
        .await
        .map(|plugins| plugins.first().cloned())
}

pub async fn badge_from_id(badge_id: i64, cookie: Option<&str>) -> RobloxResult<Badge> {
    api_helper::get(
        format!("https://badges.roblox.com/v1/badges/{}", badge_id),
        cookie
    )
        .await
        .map_async(api_helper::deserialize_body)
        .await
}
