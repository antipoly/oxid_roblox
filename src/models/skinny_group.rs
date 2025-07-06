use crate::derives::GroupDerive;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinnyGroup {
  pub id: i64,
  pub name: String,
  pub has_verified_badge: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinnyGroupWithMemberCount {
    pub id: i64,
    pub name: String,
    pub member_count: i64,
    pub has_verified_badge: bool,
}

impl GroupDerive for SkinnyGroup {
    fn id(&self) -> i64 {
        self.id
    }
}
