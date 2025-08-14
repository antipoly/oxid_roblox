use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SkinnyRole {
  pub id: i64,
  pub name: String,
  pub rank: i64,
}
