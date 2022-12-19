mod db;

use tokio;
use clap::{self, Parser};
use serde_json;

const BASEURL: &str = "https://members-ng.iracing.com";

#[derive(clap::Parser)]
struct Args {
    /// Rebuild the database from scratch
    #[arg(short, long)]
    rebuild_db: bool,

    /// Sync driver to db
    #[arg(short = 'D', long)]
    sync_driver_to_db: Option<String>,
}

fn has_async(args: &Args) -> bool {
    return args.sync_driver_to_db.is_some();
}

async fn auth(client: &reqwest::Client) {
    let user = std::env::var("IRACING_USER").unwrap();
    let token = std::env::var("IRACING_TOKEN").unwrap();

    let body = serde_json::json!(
        {"email": user, "password": token}
    );

    let response = client.post(BASEURL.to_owned() + "/auth").json(&body).send().await.unwrap();
    assert!(response.status() == reqwest::StatusCode::OK);
}

fn tokio_main(args: &Args) {
    if !has_async(&args) {
        return;
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let client = reqwest::Client::builder().cookie_store(true).build().unwrap();

        auth(&client).await;
    });
}

fn main() {
    let args = Args::parse();

    if args.rebuild_db {
        db::rebuild_db();
    }

    tokio_main(&args);
}
