use async_trait::async_trait;

#[async_trait]
pub trait System {
    async fn get_tasks(&self);
}

pub trait Task {
    fn get_title(&self) -> String;
    fn get_description(&self) -> String;
    fn get_status(&self) -> String;
}
