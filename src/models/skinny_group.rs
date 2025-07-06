use crate::derives::GroupDerive;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SkinnyGroup {
    pub id: i64,
    pub name: String,
    pub has_verified_badge: bool,
}

impl GroupDerive for SkinnyGroup {
    fn id(&self) -> i64 {
        self.id
    }
}
