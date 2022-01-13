use std::{env, fs};

use chrono::{DateTime, TimeZone, Utc};
use futures::{stream::FuturesUnordered, StreamExt};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let lockfile_str = fs::read_to_string(&args[1]).unwrap();

    let lockfile: Value = serde_json::from_str(&lockfile_str).unwrap();

    let urls = generate_urls(lockfile);

    let client = reqwest::Client::builder()
        .user_agent("npm-package-age/0.1.0 (+https://github.com/lannonbr/npm-package-age)")
        .build()
        .unwrap();

    let mut requests = FuturesUnordered::new();

    let mut packages: Vec<Package> = vec![];

    for req in urls {
        let client = client.clone();
        requests.push(tokio::spawn(async move {
            client.get(req).send().await.unwrap().text().await.unwrap()
        }));

        if requests.len() > 20 {
            let package_registry_details_str = requests.next().await.unwrap().unwrap();

            let entry: Value = serde_json::from_str(&package_registry_details_str).unwrap();

            let name = entry["name"].as_str().unwrap().to_string();
            let publishes = get_publishes(entry);

            packages.push(Package { name, publishes });
        }
    }

    while let Some(npm_resp) = requests.next().await {
        let package_registry_details_str = npm_resp.unwrap();

        let entry: Value = serde_json::from_str(&package_registry_details_str).unwrap();

        let name = entry["name"].as_str().unwrap().to_string();
        let publishes = get_publishes(entry);

        packages.push(Package { name, publishes });
    }

    parse_packages(&packages);

    Ok(())
}

fn generate_urls(lockfile: Value) -> Vec<String> {
    let lockfile_version = match &lockfile["lockfileVersion"] {
        Value::Number(n) => n.as_u64().unwrap(),
        _ => panic!("No lockfileversion field, breaking"),
    };

    let urls: Vec<String> = if lockfile_version == 1 {
        let deps = &lockfile["dependencies"].as_object().unwrap();

        let deps_keys: Vec<String> = deps.keys().map(|f| f.to_owned()).collect();

        let urls: Vec<String> = deps_keys
            .iter()
            .map(|key| format!("https://registry.npmjs.org/{}", key))
            .collect();

        urls
    } else if lockfile_version == 2 {
        let deps = &lockfile["packages"].as_object().unwrap();

        let deps_keys: Vec<String> = deps
            .keys()
            .map(|key| key.to_owned())
            .filter(|key| key.len() > 0)
            .map(|key| key.split_once("node_modules/").unwrap().1.to_owned())
            .collect();

        let urls: Vec<String> = deps_keys
            .iter()
            .map(|key| format!("https://registry.npmjs.org/{}", key))
            .collect();

        urls
    } else {
        panic!("Unsupported lockfile version");
    };

    urls
}

fn get_publishes(entry: Value) -> serde_json::Map<String, Value> {
    let publishes = entry["time"].as_object().unwrap();

    publishes.to_owned()
}

fn parse_packages(packages: &Vec<Package>) {
    for pkg in packages {
        let publishes_arr: Vec<(&String, i64)> = pkg
            .publishes
            .iter()
            .map(|(k, v)| {
                (
                    k,
                    v.as_str()
                        .unwrap()
                        .parse::<DateTime<Utc>>()
                        .unwrap()
                        .timestamp_millis(),
                )
            })
            .filter(|(k, _)| k.clone() != "created" || k.clone() != "modified")
            .collect();

        let max = publishes_arr.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
        println!(
            "Latest publish for {}: {}",
            pkg.name,
            Utc.timestamp_millis(max.1).format("%+")
        );
    }
}

struct Package {
    name: String,
    publishes: serde_json::Map<String, Value>,
}
