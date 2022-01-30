use futures::{stream::FuturesUnordered, StreamExt};
use serde_json::Value;
use std::{collections::HashSet, fs, path};

use crate::structs::Package;

pub async fn fetch_lockfile(input: String, client: &reqwest::Client) -> Value {
    let lockfile: Value = if input.starts_with("http") && input.ends_with("package-lock.json") {
        let lockfile_str = client
            .get(input)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        serde_json::from_str(&lockfile_str)
            .unwrap_or_else(|err| panic!("Error parsing url result {}", err))
    } else if path::Path::new(&input).exists() {
        let lockfile_str = fs::read_to_string(input).unwrap();
        serde_json::from_str(&lockfile_str)
            .unwrap_or_else(|err| panic!("Error parsing file {}", err))
    } else {
        panic!("Not a URL or path to a pacakge-lock.json file");
    };
    lockfile
}

pub fn generate_urls(lockfile: Value) -> Vec<String> {
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

        let deps_keys: HashSet<String> = deps
            .keys()
            .map(|key| key.to_owned())
            .filter(|key| !key.is_empty())
            .map(|key| key.rsplit_once("node_modules/").unwrap().1.to_owned())
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

pub fn get_publishes(entry: Value) -> serde_json::Map<String, Value> {
    let publishes = entry["time"].as_object().unwrap();

    publishes.to_owned()
}

pub async fn get_package_lock_jsons(urls: Vec<String>, client: reqwest::Client) -> Vec<Package> {
    let mut requests = FuturesUnordered::new();

    let mut packages: Vec<Package> = Vec::new();

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
    packages
}
