#[macro_use] extern crate rocket;

mod server;
mod response_timer;
mod schema;
mod db;
mod iracing_client;
mod category_type;
mod event_type;
mod driverid;
mod motec_xml;

use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Args {
    /// Rebuild the database from scratch
    #[arg(short, long)]
    rebuild_db: bool,

    /// Rebuild the database, but only the schema
    #[arg(long)]
    rebuild_db_schema: bool,

    /// Add missing cached sessions to the database
    #[arg(short, long)]
    update_db: bool,

    /// Sync driver to db
    #[arg(short = 'D', long)]
    sync_drivers_to_db: Vec<String>,

    /// Sync driver to db (current season only)
    #[arg(short = 'd', long)]
    sync_drivers_to_db_partial: Vec<String>,

    /// Sync cust_ids to db 
    #[arg(short = 'C', long)]
    sync_cust_ids_to_db: Vec<i64>,

    /// Sync cust_ids to db (current season only)
    #[arg(short = 'c', long)]
    sync_cust_ids_to_db_partial: Vec<i64>,

    /// Sync car infos (v as in vehicle)
    #[arg(short = 'v', long)]
    sync_car_infos_to_db: bool,

    /// Sync track infos
    #[arg(short = 't', long)]
    sync_track_infos_to_db: bool,

    /// Sync season infos
    #[arg(short = 's', long)]
    sync_season_infos_to_db: bool,

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
    #[arg(long = "server")]
    start_server: bool,

    /// Do the motec thing
    #[arg(short = 'm', long = "motec")]
    motec_thing: bool,
}

fn has_async(args: &Args) -> bool {
    return
        !args.sync_drivers_to_db.is_empty() ||
        !args.sync_cust_ids_to_db.is_empty() ||
        !args.sync_drivers_to_db_partial.is_empty() ||
        !args.sync_cust_ids_to_db_partial.is_empty() ||
        args.season_year.is_some() ||
        args.sync_car_infos_to_db ||
        args.sync_track_infos_to_db ||
        args.sync_season_infos_to_db;
}

async fn tokio_main(args: &Args) {
    if !has_async(&args) {
        return;
    }

    let mut client = iracing_client::IRacingClient::new();

    client.auth().await;

    if !args.sync_drivers_to_db.is_empty() || !args.sync_drivers_to_db_partial.is_empty() {
        iracing_client::sync_drivers_to_db(&mut client, &args.sync_drivers_to_db, &args.sync_drivers_to_db_partial).await;
    }

    if !args.sync_cust_ids_to_db.is_empty() || !args.sync_cust_ids_to_db_partial.is_empty() {
        iracing_client::sync_cust_ids_to_db(&mut client, &args.sync_cust_ids_to_db, &args.sync_cust_ids_to_db_partial).await;
    }

    if args.season_year.is_some() && args.season_quarter.is_some() {
        iracing_client::sync_season_to_db(&mut client,
            args.season_year.unwrap(), args.season_quarter.unwrap(), args.season_week).await;
    }

    if args.sync_car_infos_to_db {
        iracing_client::sync_car_infos_to_db(&mut client).await;
    }

    if args.sync_track_infos_to_db {
        iracing_client::sync_track_infos_to_db(&mut client).await;
    }

    if args.sync_season_infos_to_db {
        iracing_client::sync_season_infos_to_db(&mut client).await;
    }
}

#[rocket::main]
async fn main() {
    let args = Args::parse();

    fs::create_dir_all(crate::db::get_sessions_dir()).unwrap();

    if args.motec_thing {
        crate::motec_xml::output_motec_track_xmls();
    }
    if args.rebuild_db_schema {
        db::rebuild_db_schema();
    }
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
