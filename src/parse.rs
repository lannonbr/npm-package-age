use std::io;

use crate::{
    structs::{Output, Package},
    Opt,
};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
struct ParsedPackage {
    name: String,
    max_version: String,
    timestamp: i64,
}

pub fn parse_packages(packages: Vec<Package>, args: Opt) {
    let mut sorted: Vec<ParsedPackage> = packages
        .iter()
        .map(|pkg| {
            let publishes_arr: Vec<(&String, i64)> = pkg
                .publishes
                .iter()
                .map(|(k, v)| (k, timestamp_millis(v)))
                .filter(|(k, _)| {
                    k.as_str() != "created" && k.as_str() != "modified" && k.as_str() != "0.0.0"
                })
                .collect();

            let max = publishes_arr
                .iter()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap();
            ParsedPackage {
                name: pkg.name.clone(),
                max_version: max.0.to_owned(),
                timestamp: max.1,
            }
        })
        .collect();

    if args.reverse {
        sorted.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    } else {
        sorted.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    }

    match args.format {
        Output::CSV => {
            let mut wtr = csv::Writer::from_writer(io::stdout());
            for p in sorted {
                wtr.serialize(p).unwrap();
            }
            wtr.flush().unwrap();
        }
        Output::JSON => {
            let out = serde_json::to_string_pretty(&sorted).unwrap();
            println!("{out}");
        }
    }
}

fn timestamp_millis(v: &serde_json::Value) -> i64 {
    v.as_str()
        .unwrap()
        .parse::<DateTime<Utc>>()
        .unwrap()
        .timestamp_millis()
}
