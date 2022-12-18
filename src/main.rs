use std::fs;
use serde_json;
use rusqlite;

// const SESSIONS_DIR: &str = "data/sessions";
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

    return DbContext {
        tx,
        insert_track_statement,
        insert_track_config_statement,
        insert_car_statement,
    };
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

fn parse_session_zip(zip_file: &str) {
    let contents = read_single_file_zip(zip_file);
    let data: serde_json::Value = serde_json::from_str(&contents).unwrap();

    println!("{}", data["subsession_id"]);
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

fn rebuild_db() {
    fs::remove_file(SQLITE_DB_FILE).unwrap();

    let mut con = rusqlite::Connection::open(SQLITE_DB_FILE).unwrap();
    let mut tx = con.transaction().unwrap();

    {
        build_db_schema(&tx);
        let mut ctx = create_db_context(&mut tx);
        rebuild_tracks(&mut ctx);
        rebuild_cars(&mut ctx);
    }

    tx.commit().unwrap();
}


fn main() {
    rebuild_db();
}
