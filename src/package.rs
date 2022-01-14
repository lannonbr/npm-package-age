#[derive(Clone)]
pub struct Package {
    pub name: String,
    pub publishes: serde_json::Map<String, serde_json::Value>,
}
