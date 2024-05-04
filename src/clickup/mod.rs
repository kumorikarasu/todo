mod task;

use async_trait::async_trait;

use task::Task;
use crate::{config, traits};
use crate::traits::System;

pub fn new() -> ClickUp {
    ClickUp::new(config::get_api_key())
}

pub struct ClickUp { 
    api_key: String,
}

impl ClickUp {
    pub fn new(api_key: String) -> ClickUp {
        ClickUp {
            api_key,
        }
    }
}

#[async_trait]
impl System for ClickUp {
    async fn get_tasks(&self) {
        println!("get tasks");
        let client = reqwest::Client::builder().build().unwrap();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Authorization", self.api_key.parse().unwrap());

        let data = "";

        let request = client.request(reqwest::Method::GET, "https://api.clickup.com/api/v2/list/list_id/task")
            .headers(headers)
            .body(data);

        let response = request.send().await;

        println!("{:?}", response);

        let body = response.unwrap().text().await;

        println!("{:?}", body);


        /*
        let mut tasks: Vec<Box<Task>> = Vec::new();
        tasks.push(Box::new(Task::new("task1".to_string())));
        tasks.push(Box::new(Task::new("task2".to_string())));

        println!("tasks: {:?}", tasks);

        tasks.into_iter().map(|x| x as Box<dyn crate::traits::Task>).collect()
        */
    }
}

