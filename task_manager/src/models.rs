use crate::prelude::*;

/// Represent a singular task.
#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub tags: HashSet<String>,
    pub status: Status,
}

#[derive(Debug, Clone)]
pub enum Status {
    Upcoming,
    InProgress,
    Complete,
    StandBy,
}
