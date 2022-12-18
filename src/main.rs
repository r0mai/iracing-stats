use std::{fs, path::PathBuf};
use serde_json;
use rusqlite;
use chrono::{self, TimeZone};

const SESSIONS_DIR: &str = "data/sessions";
const TRACK_DATA_FILE: &str = "data/tracks.json";
const CAR_DATA_FILE: &str = "data/cars.json";
const SQLITE_DB_FILE: &str = "stats.db";
const SCHEMA_SQL: &str = "schema.sql";
// const BASEURL: &str = "https://members-ng.iracing.com";

struct DbContext<'a> {
    tx: &'a rusqlite::Transaction<'a>,
    insert_track_statement: rusqlite::Statement<'a>,
    insert_track_config_statement: rusqlite::Statement<'a>,
    insert_car_statement: rusqlite::Statement<'a>,
    insert_subsession_statement: rusqlite::Statement<'a>,
}

fn create_db_context<'a>(tx: &'a mut rusqlite::Transaction) -> DbContext<'a> {
    let insert_track_statement = tx.prepare(r#"
        INSERT OR IGNORE INTO track VALUES(
            ?, /* package_id */
            ?  /* track_name */
        );"#).unwrap();
    let insert_track_config_statement = tx.prepare(r#"
        INSERT INTO track_config VALUES(
            ?, /* track_id */
            ?, /* package_id */
            ?, /* config_name */
            ?  /* track_config_length */
        );"#).unwrap();
    let insert_car_statement = tx.prepare(r#"
        INSERT INTO car VALUES(
            ?, /* car_id */
            ?, /* car_name */
            ?  /* car_name_abbreviated */
        );"#).unwrap();
    let insert_subsession_statement = tx.prepare(r#"
        INSERT INTO subsession VALUES(
            ?, /* subsession_id */
            ?, /* session_id */
            ?, /* start_time */
            ?, /* license_category_id */
            ?  /* track_id */
        );"#).unwrap();

    return DbContext {
        tx,
        insert_track_statement,
        insert_track_config_statement,
        insert_car_statement,
        insert_subsession_statement
    };
}

fn parse_date(str: &str) -> chrono::DateTime<chrono::Utc> {
    let naive = chrono::NaiveDateTime::parse_from_str(str, "%Y-%m-%dT%H:%M:%SZ").unwrap();
    return chrono::Utc.from_local_datetime(&naive).unwrap();
}

fn read_single_file_zip(file_name: &str) -> String {
    let zip_file = fs::File::open(file_name).unwrap();
    let mut archive = zip::ZipArchive::new(zip_file).unwrap();

    if archive.len() != 1 {
        return "".to_owned();
    }

    let mut session_file = archive.by_index(0).unwrap();

    return std::io::read_to_string(&mut session_file).unwrap();
}

fn read_json_zip(zip_file: &str) -> serde_json::Value {
    let contents = read_single_file_zip(zip_file);
    let data: serde_json::Value = serde_json::from_str(&contents).unwrap();

    return data;
}

fn build_db_schema(tx: &rusqlite::Transaction) {
    let schema_sql = fs::read_to_string(SCHEMA_SQL).unwrap();
    tx.execute_batch(&schema_sql).unwrap();
}

fn add_track_to_db(ctx: &mut DbContext, track: &serde_json::Value) {
    ctx.insert_track_statement.execute((
        track["package_id"].as_u64().unwrap(),
        track["track_name"].as_str().unwrap()
    )).unwrap();
    ctx.insert_track_config_statement.execute((
        track["track_id"].as_u64().unwrap(),
        track["package_id"].as_u64().unwrap(),
        track["config_name"].as_str().unwrap_or(""),
        track["track_config_length"].as_f64().unwrap()
    )).unwrap();
}

fn add_car_to_db(ctx: &mut DbContext, car: &serde_json::Value) {
    ctx.insert_car_statement.execute((
        car["car_id"].as_u64().unwrap(),
        car["car_name"].as_str().unwrap(),
        car["car_name_abbreviated"].as_str().unwrap(),
    )).unwrap();
}

fn add_subsession_to_db(ctx: &mut DbContext, subsession: &serde_json::Value) {
    let subsession_id = subsession["subsession_id"].as_u64().unwrap();

    ctx.insert_subsession_statement.execute((
        subsession_id,
        subsession["session_id"].as_u64().unwrap(),
        parse_date(subsession["start_time"].as_str().unwrap()),
        subsession["license_category_id"].as_u64().unwrap(),
        subsession["track"]["track_id"].as_u64().unwrap()
    )).unwrap();
}

fn add_sessions_to_db<I>(ctx: &mut DbContext, files: I) 
    where I: Iterator<Item = PathBuf>
{
    let mut i = 0;
    for session_file in files {
        if session_file.extension().unwrap_or_default() != "zip" {
            continue;
        }
        if i % 1000 == 0 {
            println!("Progress: {}", i);
        }
        i += 1;

        let data = read_json_zip(session_file.to_str().unwrap());
        add_subsession_to_db(ctx, &data);
    }
}

fn rebuild_tracks(ctx: &mut DbContext) {
    let contents = fs::read_to_string(TRACK_DATA_FILE).unwrap();
    let tracks: serde_json::Value = serde_json::from_str(&contents).unwrap();

    for track in tracks.as_array().unwrap() {
        add_track_to_db(ctx, &track);
    }
}

fn rebuild_cars(ctx: &mut DbContext) {
    let contents = fs::read_to_string(CAR_DATA_FILE).unwrap();
    let cars: serde_json::Value = serde_json::from_str(&contents).unwrap();

    for car in cars.as_array().unwrap() {
        add_car_to_db(ctx, &car);
    }
}

fn rebuild_sessions(ctx: &mut DbContext) {
    let paths = fs::read_dir(SESSIONS_DIR).unwrap();
    add_sessions_to_db(ctx, paths.map(|e| e.unwrap().path()));
}

fn rebuild_db() {
    fs::remove_file(SQLITE_DB_FILE).unwrap();

    let mut con = rusqlite::Connection::open(SQLITE_DB_FILE).unwrap();
    let mut tx = con.transaction().unwrap();

    {
        build_db_schema(&tx);
        let mut ctx = create_db_context(&mut tx);
        rebuild_tracks(&mut ctx);
        rebuild_cars(&mut ctx);
        rebuild_sessions(&mut ctx);
    }

    tx.commit().unwrap();
}


fn main() {
    rebuild_db();
}
