use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the current working context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    /// The main note describing what the user is working on
    pub note: String,

    /// When this context was created/updated
    pub timestamp: DateTime<Utc>,

    /// Current git repository (if in a git repo)
    pub repo: Option<String>,

    /// Current git branch (if in a git repo)
    pub branch: Option<String>,
}

impl Context {
    /// Create a new context with the given note
    pub fn new(note: String) -> Self {
        Self {
            note,
            timestamp: Utc::now(),
            repo: None,
            branch: None,
        }
    }

    /// Create a new context with git information
    pub fn new_with_git(note: String, repo: String, branch: String) -> Self {
        Self {
            note,
            timestamp: Utc::now(),
            repo: Some(repo),
            branch: Some(branch),
        }
    }
}
