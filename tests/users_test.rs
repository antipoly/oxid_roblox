//!

use dotenv::dotenv;
use oxid_roblox::derives::UserDerive;
use std::env;

#[tokio::test]
async fn base() {
  let user = oxid_roblox::user_from_id(1).await;
  assert!(user.is_ok());

  let user_1 = oxid_roblox::user_from_id(0).await;
  assert!(user_1.is_err());
}

#[tokio::test]
async fn auth() {
  dotenv().ok();
  let cookie = env::var("COOKIE").expect("No cookie found in .env");

  let user = oxid_roblox::authenticated_user(Some(cookie)).await;
  assert!(user.is_ok());
}

#[tokio::test]
async fn group_roles() {
  let user = oxid_roblox::user_from_id(1).await.expect("User not found");
  let roles = user.group_roles().await;
  assert!(roles.is_ok());
}

#[tokio::test]
async fn role_in_group() {
  let user = oxid_roblox::user_from_id(1).await.expect("User not found");
  let roles = user.role_in_group(127081).await;
  assert!(roles.is_ok());

  let roles_1 = user.role_in_group(1).await;
  assert!(roles_1.is_err());
}
