use std::env;

mod fetch;
mod package;
mod parse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let client = reqwest::Client::builder()
        .user_agent("npm-package-age/0.1.0 (+https://github.com/lannonbr/npm-package-age)")
        .build()
        .unwrap();

    let input = &args[1];

    let lockfile = fetch::fetch_lockfile(input, &client).await;

    let urls = fetch::generate_urls(lockfile);

    let packages = fetch::get_package_lock_jsons(urls, client).await;

    parse::parse_packages(packages);

    Ok(())
}
