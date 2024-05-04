#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Config {
    pub system: String
}

impl Config {
    pub fn new(system: String) -> Config {
        Config {
            system: "clickup".to_string()
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Secrets {

    #[serde(rename = "apiKey")]
    pub api_key: String,
}
