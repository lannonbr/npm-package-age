use clap::StructOpt;
use structs::Output;

mod fetch;
mod parse;
mod structs;

/// A rust program to examine npm lockfiles and print out the last published date of each package
#[derive(StructOpt, Debug, Clone)]
pub struct Opt {
    /// package-lock you want to examine
    #[structopt(long)]
    file: String,

    /// Output format
    #[structopt(long, default_value = "csv", possible_values = ["csv", "json"])]
    format: Output,

    /// View items in descending chronological order
    #[structopt(long)]
    reverse: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::parse();

    let client = reqwest::Client::builder()
        .user_agent("npm-package-age/0.1.0 (+https://github.com/lannonbr/npm-package-age)")
        .build()
        .unwrap();

    let input = args.file.clone();

    let lockfile = fetch::fetch_lockfile(input, &client).await;

    let urls = fetch::generate_urls(lockfile);

    let packages = fetch::get_package_lock_jsons(urls, client).await;

    parse::parse_packages(packages, args.clone());

    Ok(())
}
