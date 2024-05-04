use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Closed,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for TaskStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Todo" => Ok(Self::Todo),
            "InProgress" => Ok(Self::InProgress),
            "Done" => Ok(Self::Done),
            "Closed" => Ok(Self::Closed),
            _ => Err(()),
        }
    }
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::Todo
    }
}

impl Task {
    pub fn new(title: String) -> Self { 
        Self { 
            id: None,
            title,
            description: None,
            status: TaskStatus::default() 
        }
    }

    pub fn new_id(id: i64, title: String) -> Self { 
        Self { 
            id: Some(id),
            title,
            description: None,
            status: TaskStatus::default() 
        }
    }

    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    pub(crate) fn set_id(&mut self, id: i64) -> () {
        self.id = Some(id as i64);
    }
}


