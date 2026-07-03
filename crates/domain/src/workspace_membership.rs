use super::ActorReference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceMembership {
    pub id: String,
    pub workspace_id: String,
    pub actor_ref: ActorReference,
    pub status: MembershipStatus,
    pub joined_at: String,
    pub revoked_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipStatus {
    Active,
    Invited,
    Revoked,
}

impl WorkspaceMembership {
    pub fn is_active(&self) -> bool {
        self.status == MembershipStatus::Active
    }
}
