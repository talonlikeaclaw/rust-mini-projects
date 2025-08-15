use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// Represent a singular task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub tags: HashSet<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Upcoming,
    InProgress,
    Complete,
    StandBy,
}
