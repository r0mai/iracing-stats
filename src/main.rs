mod db;
mod iracing_client;

use tokio;
use clap::{self, Parser};

#[derive(clap::Parser)]
struct Args {
    /// Rebuild the database from scratch
    #[arg(short, long)]
    rebuild_db: bool,

    /// Add missing cached sessions to the database
    #[arg(short, long)]
    update_db: bool,

    /// Sync driver to db
    #[arg(short = 'D', long)]
    sync_drivers_to_db: Vec<String>,
}

fn has_async(args: &Args) -> bool {
    return !args.sync_drivers_to_db.is_empty();
}

fn tokio_main(args: &Args) {
    if !has_async(&args) {
        return;
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let client = reqwest::Client::builder().cookie_store(true).build().unwrap();

        iracing_client::auth(&client).await;

        if !args.sync_drivers_to_db.is_empty() {
            iracing_client::sync_drivers_to_db(&client, &args.sync_drivers_to_db).await;
        }
    });
}

fn main() {
    let args = Args::parse();

    if args.rebuild_db {
        db::rebuild_db();
    }
    if args.update_db {
        db::update_db();
    }

    tokio_main(&args);
}
