use std::str::FromStr;

#[derive(Clone)]
pub struct Package {
    pub name: String,
    pub publishes: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub enum Output {
    CSV,
    JSON,
}

impl FromStr for Output {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "csv" => Ok(Output::CSV),
            "json" => Ok(Output::JSON),
            _ => Err("Unknown file type"),
        }
    }
}
