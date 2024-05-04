
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TodoError {
    message: String,
}

impl TodoError {
    pub fn to_string(&self) -> String {
        self.message.clone()
    }
}

impl std::fmt::Display for TodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<T> From<T> for TodoError where T: std::error::Error + Send + Sync + 'static {
    fn from(err: T) -> TodoError {
        TodoError {
            message: err.to_string(),
        }
    }
}
