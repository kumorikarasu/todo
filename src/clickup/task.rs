use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    name: String,
    description: String,
    assignees: Vec<u32>,
    tags: Vec<String>,
    status: String,
    priority: u32,
    due_date: u64,
    due_date_time: bool,
    time_estimate: u64,
    start_date: u64,
    start_date_time: bool,
    notify_all: bool,
    parent: Option<u32>,
    links_to: Option<u32>,
    custom_fields: Vec<CustomField>,
}

impl Task {
    pub fn new(to_string: String) -> Self {
        Task {
            name: to_string,
            description: "".to_string(),
            assignees: vec![],
            tags: vec![],
            status: "".to_string(),
            priority: 0,
            due_date: 0,
            due_date_time: false,
            time_estimate: 0,
            start_date: 0,
            start_date_time: false,
            notify_all: false,
            parent: None,
            links_to: None,
            custom_fields: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct CustomField {
    id: String,
    value: String,
}

impl crate::traits::Task for Task {
    fn get_title(&self) -> String {
        todo!()
    }

    fn get_description(&self) -> String {
        todo!()
    }

    fn get_status(&self) -> String {
        todo!()
    }
}
