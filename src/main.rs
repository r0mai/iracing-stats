#[macro_use] extern crate rocket;

mod server;
mod db;
mod iracing_client;
mod category_type;
mod event_type;

use clap::Parser;
use std::fs;

#[derive(Parser)]
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

    /// Sync car infos
    #[arg(short = 'c', long)]
    sync_car_infos: bool,

    /// Sync track infos
    #[arg(short = 't', long)]
    sync_track_infos: bool,

    /// Sync season year to db
    #[arg(short = 'y', long)]
    season_year: Option<i32>,

    /// Sync season year to db
    #[arg(short = 'q', long)]
    season_quarter: Option<i32>,

    /// Sync season year to db
    #[arg(short = 'w', long)]
    season_week: Option<i32>,

    /// Start server
    #[arg(short = 's', long = "server")]
    start_server: bool
}

fn has_async(args: &Args) -> bool {
    return
        !args.sync_drivers_to_db.is_empty() ||
        args.season_year.is_some() ||
        args.sync_car_infos ||
        args.sync_track_infos;
}

async fn tokio_main(args: &Args) {
    if !has_async(&args) {
        return;
    }

    let mut client = iracing_client::IRacingClient::new();

    client.auth().await;

    if !args.sync_drivers_to_db.is_empty() {
        iracing_client::sync_drivers_to_db(&mut client, &args.sync_drivers_to_db).await;
    }

    if args.season_year.is_some() && args.season_quarter.is_some() {
        iracing_client::sync_season_to_db(&mut client,
            args.season_year.unwrap(), args.season_quarter.unwrap(), args.season_week).await;
    }

    if args.sync_car_infos {
        iracing_client::sync_car_infos(&mut client).await;
    }

    if args.sync_track_infos {
        iracing_client::sync_track_infos(&mut client).await;
    }
}

#[rocket::main]
async fn main() {
    let args = Args::parse();

    fs::create_dir_all(crate::db::SESSIONS_DIR).unwrap();

    if args.rebuild_db {
        db::rebuild_db();
    }
    if args.update_db {
        db::update_db();
    }
    if args.start_server {
        crate::server::start_rocket_server().await;
    } else {
        tokio_main(&args).await;
    }
}
