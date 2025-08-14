//!

use dotenv::dotenv;
use oxid_roblox::derives::{GroupDerive};
use std::env;

#[tokio::test]
async fn base() {
  let group = oxid_roblox::group_from_id(1).await;
  assert!(group.is_ok());

  let group_1 = oxid_roblox::user_from_id(0).await;
  assert!(group_1.is_err());
}

#[tokio::test]
async fn roles() {
  dotenv().ok();
  let cookie = env::var("COOKIE").expect("No cookie found in .env");

  let group = oxid_roblox::group_from_id(7103605).await.expect("Group not found");
  let roles = group.roles(Some(&cookie)).await;
  assert!(roles.is_ok());
}

