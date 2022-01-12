use std::{env, fs};

use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let input = fs::read_to_string(&args[1]).unwrap();

    let val: Value = serde_json::from_str(&input).unwrap();

    let lockfile_version = match &val["lockfileVersion"] {
        Value::Number(n) => n.as_u64().unwrap(),
        _ => panic!("No lockfileversion field, breaking"),
    };

    if lockfile_version == 1 {
        println!("Lockfile version 1");
    }

    let deps = &val["dependencies"].as_object().unwrap();

    let deps_keys: Vec<String> = deps.keys().map(|f| f.to_owned()).collect();

    let urls: Vec<String> = deps_keys
        .iter()
        .map(|key| format!("https://registry.npmjs.org/{}", key))
        .collect();

    dbg!(urls);

    Ok(())
}
