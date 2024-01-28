#[macro_use] extern crate rocket;

mod server;
mod server_logger;
mod schema;
mod db;
mod iracing_client;
mod category_type;
mod event_type;
mod simsession_type;
mod driverid;
mod motec_xml;
mod discord_bot;
mod discord_hook;
mod dirs;
mod sof_calculator;

use clap::Parser;
use sof_calculator::SofCalculator;
use std::fs;
use sha2::Digest;
use sha2::Sha256;

#[derive(Parser)]
struct Args {
    /// Rebuild the database from scratch
    #[arg(short, long)]
    rebuild_db: bool,

    /// Rebuild the database, but only the schema
    #[arg(long)]
    rebuild_db_schema: bool,

    /// Rebuild site teams
    #[arg(long)]
    rebuild_site_teams: bool,

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

    /// Sync site teams to db
    #[arg(long)]
    sync_site_teams_to_db: bool,

    /// Sync site teams to db (current season only)
    #[arg(long)]
    sync_site_teams_to_db_partial: bool,

    /// Sync car & car class infos (v as in vehicle)
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

    /// Use HTTPS when running the server
    #[arg(long = "enable-https")]
    enable_https: bool,

    /// Do the motec thing
    #[arg(short = 'm', long = "motec")]
    motec_thing: bool,

    /// Discord hook test
    #[arg(long)]
    send_discord_update: bool,

    /// Test Discord hook test
    #[arg(long)]
    test_send_discord_update: bool,

    /// Generate iracing token
    #[arg(long)]
    generate_iracing_token: bool,

    /// User (e-mail) for iracing token
    #[arg(long)]
    gen_email: Option<String>,

    /// PW for iracing token
    #[arg(long)]
    gen_pw: Option<String>,
}

fn has_async(args: &Args) -> bool {
    return
        !args.sync_drivers_to_db.is_empty() ||
        !args.sync_cust_ids_to_db.is_empty() ||
        !args.sync_drivers_to_db_partial.is_empty() ||
        !args.sync_cust_ids_to_db_partial.is_empty() ||
        args.sync_site_teams_to_db ||
        args.sync_site_teams_to_db_partial ||
        args.season_year.is_some() ||
        args.sync_car_infos_to_db ||
        args.sync_track_infos_to_db ||
        args.sync_season_infos_to_db ||
        args.test_send_discord_update;
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

    if args.sync_site_teams_to_db {
        iracing_client::sync_site_teams_to_db(&mut client, false).await;
    }

    if args.test_send_discord_update {
        discord_hook::send_discord_update(vec![63740038, 61486453, 61145537, 13059307, 64483246], true).await;
        // discord_hook::send_discord_update(vec![16936417i64], true).await; // This one has a weird reason_out
    }

    if args.sync_site_teams_to_db_partial {
        let subsession_ids = iracing_client::sync_site_teams_to_db(&mut client, true).await;
        if args.send_discord_update {
            discord_hook::send_discord_update(subsession_ids, false).await;
        }
    }

    if args.season_year.is_some() && args.season_quarter.is_some() {
        iracing_client::sync_season_to_db(&mut client,
            args.season_year.unwrap(), args.season_quarter.unwrap(), args.season_week).await;
    }

    if args.sync_car_infos_to_db {
        iracing_client::sync_car_infos_to_db(&mut client).await;
        iracing_client::sync_car_class_infos_to_db(&mut client).await;
    }

    if args.sync_track_infos_to_db {
        iracing_client::sync_track_infos_to_db(&mut client).await;
    }

    if args.sync_season_infos_to_db {
        iracing_client::sync_season_infos_to_db(&mut client).await;
    }
}

fn encode_iracing_pw(password: &str, identifier: &str) -> String {
    let mut hasher = Sha256::new();
    let normalized = identifier.trim().to_lowercase();

    hasher.update(format!("{password}{normalized}"));
    base64::encode(hasher.finalize())
}


#[rocket::main]
async fn main() {
    let args = Args::parse();

    fs::create_dir_all(crate::db::get_sessions_dir()).unwrap();

    if args.motec_thing {
        crate::motec_xml::output_motec_track_xmls2();
        crate::motec_xml::output_motec_car_xmls2();
    }
    if args.rebuild_db_schema {
        db::rebuild_db_schema();
    }
    if args.rebuild_db {
        db::rebuild_db();
    }
    if args.rebuild_site_teams {
        db::rebuild_site_teams_in_db();
    }
    if args.update_db {
        db::update_db();
    }
    if args.generate_iracing_token {
        println!("{}", encode_iracing_pw(args.gen_pw.clone().unwrap().as_str(), args.gen_email.clone().unwrap().as_str()));
    }
    if args.start_server {
        crate::server::start_rocket_server(args.enable_https).await;
    } else {
        tokio_main(&args).await;
    }
}
