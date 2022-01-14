use crate::package::Package;
use chrono::{DateTime, TimeZone, Utc};

pub fn parse_packages(packages: Vec<Package>) {
    let mut sorted: Vec<(String, &String, i64)> = packages
        .iter()
        .map(|pkg| {
            let publishes_arr: Vec<(&String, i64)> = pkg
                .publishes
                .iter()
                .map(|(k, v)| (k, timestamp_millis(v)))
                .filter(|(k, _)| k.as_str() != "created" && k.as_str() != "modified")
                .collect();

            let max = publishes_arr
                .iter()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap();
            (pkg.name.clone(), max.0, max.1)
        })
        .collect();

    sorted.sort_by(|(_, _, a), (_, _, b)| a.cmp(b));

    for (pkg_name, version, timestamp) in sorted {
        println!(
            "{} {} {}",
            pkg_name,
            version,
            Utc.timestamp_millis(timestamp).format("%+")
        );
    }
}

fn timestamp_millis(v: &serde_json::Value) -> i64 {
    v.as_str()
        .unwrap()
        .parse::<DateTime<Utc>>()
        .unwrap()
        .timestamp_millis()
}
