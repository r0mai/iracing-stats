use std::{fs, path::PathBuf, path::Path, io::Write, env};
use serde_json::{self, Value};
use rusqlite;
use chrono::{self, TimeZone};
use zip::write::FileOptions;
use lazy_static::lazy_static;
use sea_query_rusqlite::RusqliteBinder;
use sea_query::{
    Query,
    Expr,
    Order,
    SqliteQueryBuilder,
    Func,
};
use crate::schema::{
    Driver,
    Session,
    Subsession,
    DriverResult,
    Simsession,
    TrackConfig,
    Track,
    Car,

    is_event_type,
    is_category_type,
    SchemaUtils,
};
use crate::event_type::EventType;
use crate::category_type::CategoryType;
use crate::driverid::DriverId;

const SESSIONS_DIR: &str = "data/sessions";
const TRACK_DATA_FILE: &str = "data/tracks.json";
const CAR_DATA_FILE: &str = "data/cars.json";
const SQLITE_DB_FILE: &str = "stats.db";
const BASE_DIR_ENV_VAR: &str = "IRACING_STATS_BASE_DIR";

fn get_base_dir() -> &'static Path {
    lazy_static! {
        static ref BASE_DIR: PathBuf = PathBuf::from(
            match env::var(BASE_DIR_ENV_VAR) {
                Ok(value) => value,
                Err(_error) => ".".to_owned()
            }
        );
    }
    return BASE_DIR.as_path();
}

pub fn get_sqlite_db_file() -> &'static Path {
    lazy_static! {
        static ref FILE: PathBuf = get_base_dir().join(SQLITE_DB_FILE);
    }
    return FILE.as_path();
}

pub fn get_track_data_file() -> &'static Path {
    lazy_static! {
        static ref FILE: PathBuf = get_base_dir().join(TRACK_DATA_FILE);
    }
    return FILE.as_path();
}

pub fn get_car_data_file() -> &'static Path {
    lazy_static! {
        static ref FILE: PathBuf = get_base_dir().join(CAR_DATA_FILE);
    }
    return FILE.as_path();
}

pub fn get_sessions_dir() -> &'static Path {
    lazy_static! {
        static ref DIR: PathBuf = get_base_dir().join(SESSIONS_DIR);
    }
    return DIR.as_path();
}

pub fn create_db_connection() -> rusqlite::Connection {
    return rusqlite::Connection::open(get_sqlite_db_file()).unwrap();
}

pub struct DbContext<'a> {
    insert_track_statement: rusqlite::Statement<'a>,
    insert_track_config_statement: rusqlite::Statement<'a>,
    insert_car_statement: rusqlite::Statement<'a>,
    insert_subsession_statement: rusqlite::Statement<'a>,
    insert_session_statement: rusqlite::Statement<'a>,
    insert_simsession_statement: rusqlite::Statement<'a>,
    insert_driver_statement: rusqlite::Statement<'a>,
    insert_driver_result_statement: rusqlite::Statement<'a>,
}

pub fn create_db_context<'a>(tx: &'a mut rusqlite::Transaction) -> DbContext<'a> {
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
            ?, /* event_type */
            ?  /* track_id */
        );"#).unwrap();
    let insert_session_statement = tx.prepare(r#"
        INSERT OR IGNORE INTO session VALUES(
            ?,  /* session_id */
            ?   /* series_name */
    );"#).unwrap();
    let insert_simsession_statement = tx.prepare(r#"
        INSERT INTO simsession VALUES(
            ?, /* subsession_id */
            ?, /* simsession_number */
            ?  /* simsession_type */
    );"#).unwrap();
    let insert_driver_statement= tx.prepare(r#"
        INSERT OR IGNORE INTO driver VALUES(
            ?, /* cust_id */
            ?  /* display_name */
    );"#).unwrap();
    let insert_driver_result_statement= tx.prepare(r#"
        INSERT INTO driver_result VALUES(
            ?, /* cust_id */
            ?, /* team_id */
            ?, /* subsession_id */
            ?, /* simsession_number */
            ?, /* oldi_rating */
            ?, /* newi_rating */
            ?, /* old_cpi */
            ?, /* new_cpi */
            ?, /* incidents */
            ?, /* laps_complete */
            ?, /* average_lap */
            ?, /* car_id */
            ?, /* finish_position */
            ?  /* finish_position_in_class */
    );"#).unwrap();

    return DbContext {
        insert_track_statement,
        insert_track_config_statement,
        insert_car_statement,
        insert_subsession_statement,
        insert_session_statement,
        insert_simsession_statement,
        insert_driver_statement,
        insert_driver_result_statement,
    };
}

fn parse_date(str: &str) -> chrono::DateTime<chrono::Utc> {
    let naive = chrono::NaiveDateTime::parse_from_str(str, "%Y-%m-%dT%H:%M:%SZ").unwrap();
    return chrono::Utc.from_local_datetime(&naive).unwrap();
}

fn read_single_file_zip(file_name: &Path) -> String {
    let zip_file = fs::File::open(file_name).unwrap();
    let mut archive = zip::ZipArchive::new(zip_file).unwrap();

    if archive.len() != 1 {
        return "".to_owned();
    }

    let mut session_file = archive.by_index(0).unwrap();

    return std::io::read_to_string(&mut session_file).unwrap();
}

fn read_json_zip(zip_file: &Path) -> Value {
    let contents = read_single_file_zip(zip_file);
    let data: Value = serde_json::from_str(&contents).unwrap();

    return data;
}

fn write_single_file_zip(zip_path: &Path, file_name: &str, content: &str) {
    let file = fs::File::create(zip_path).unwrap();
    let mut zip = zip::ZipWriter::new(&file);
    zip.start_file(file_name, FileOptions::default()).unwrap();
    write!(&mut zip, "{content}").unwrap();
    zip.finish().unwrap();
}

fn build_db_schema(tx: &rusqlite::Transaction) {
    let schema_sql = include_str!("schema.sql");
    tx.execute_batch(schema_sql).unwrap();
}

fn build_db_indices(tx: &rusqlite::Transaction) {
    let indicies_sql = include_str!("indices.sql");
    tx.execute_batch(indicies_sql).unwrap();
}

fn add_track_to_db(ctx: &mut DbContext, track: &Value) {
    ctx.insert_track_statement.execute((
        track["package_id"].as_i64().unwrap(),
        track["track_name"].as_str().unwrap()
    )).unwrap();
    ctx.insert_track_config_statement.execute((
        track["track_id"].as_i64().unwrap(),
        track["package_id"].as_i64().unwrap(),
        track["config_name"].as_str().unwrap_or(""),
        track["track_config_length"].as_f64().unwrap()
    )).unwrap();
}

fn add_car_to_db(ctx: &mut DbContext, car: &Value) {
    ctx.insert_car_statement.execute((
        car["car_id"].as_i64().unwrap(),
        car["car_name"].as_str().unwrap(),
        car["car_name_abbreviated"].as_str().unwrap(),
    )).unwrap();
}

fn add_driver_to_db(ctx: &mut DbContext, driver_result: &Value) {
    ctx.insert_driver_statement.execute((
        driver_result["cust_id"].as_i64().unwrap(),
        driver_result["display_name"].as_str().unwrap(),
    )).unwrap();
}

fn add_driver_result_to_db(ctx: &mut DbContext, subsession_id: i64, simsession_number: i64, team_id: i64, driver_result: &Value) {
    add_driver_to_db(ctx, driver_result);

    // TODO cust_id could be factored out

    ctx.insert_driver_result_statement.execute((
        driver_result["cust_id"].as_i64().unwrap(), 
        team_id,
        subsession_id,
        simsession_number,
        driver_result["oldi_rating"].as_i64().unwrap(),
        driver_result["newi_rating"].as_i64().unwrap(),
        driver_result["old_cpi"].as_f64().unwrap(),
        driver_result["new_cpi"].as_f64().unwrap(),
        driver_result["incidents"].as_i64().unwrap(),
        driver_result["laps_complete"].as_i64().unwrap(),
        driver_result["average_lap"].as_i64().unwrap(),
        driver_result["car_id"].as_i64().unwrap(),
        driver_result["finish_position"].as_i64().unwrap(),
        driver_result["finish_position_in_class"].as_i64().unwrap(),
    )).unwrap();
}

fn add_simsession_db(ctx: &mut DbContext, subsession_id: i64, simsession: &Value) {
    let simsession_number = simsession["simsession_number"].as_i64().unwrap();

    ctx.insert_simsession_statement.execute((
        subsession_id,
        simsession_number,
        simsession["simsession_type"].as_i64().unwrap()
    )).unwrap();

    for participant in simsession["results"].as_array().unwrap() {
        if participant["cust_id"].as_i64().is_some() {
            add_driver_result_to_db(ctx, subsession_id, simsession_number, 0, participant);
        } else { // team
            let team_id = participant["team_id"].as_i64().unwrap();
            for driver in participant["driver_results"].as_array().unwrap() {
                add_driver_result_to_db(ctx, subsession_id, simsession_number, team_id, driver);
            }
        }
    }
}

fn add_subsession_to_db(ctx: &mut DbContext, subsession: &Value) {
    let subsession_id = subsession["subsession_id"].as_i64().unwrap();
    let session_id = subsession["session_id"].as_i64().unwrap();

    ctx.insert_subsession_statement.execute((
        subsession_id,
        session_id,
        parse_date(subsession["start_time"].as_str().unwrap()),
        subsession["license_category_id"].as_i64().unwrap(),
        subsession["event_type"].as_i64().unwrap(),
        subsession["track"]["track_id"].as_i64().unwrap()
    )).unwrap();

    ctx.insert_session_statement.execute((
        session_id,
        subsession["series_name"].as_str().unwrap()
    )).unwrap();

    for simsession in subsession["session_results"].as_array().unwrap() {
        add_simsession_db(ctx, subsession_id, simsession);
    }
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

        let data = read_json_zip(session_file.as_path());
        add_subsession_to_db(ctx, &data);
    }
}

fn rebuild_tracks(ctx: &mut DbContext) {
    let contents = fs::read_to_string(get_track_data_file()).unwrap();
    let tracks: Value = serde_json::from_str(&contents).unwrap();

    for track in tracks.as_array().unwrap() {
        add_track_to_db(ctx, &track);
    }
}

fn rebuild_cars(ctx: &mut DbContext) {
    let contents = fs::read_to_string(get_car_data_file()).unwrap();
    let cars: Value = serde_json::from_str(&contents).unwrap();

    for car in cars.as_array().unwrap() {
        add_car_to_db(ctx, &car);
    }
}

fn rebuild_sessions(ctx: &mut DbContext) {
    let paths = fs::read_dir(get_sessions_dir()).unwrap();
    add_sessions_to_db(ctx, paths.map(|e| e.unwrap().path()));
}

pub fn add_session_to_db_from_cache(ctx: &mut DbContext, subsession_id: i64) {
    add_subsession_to_db(ctx, &read_json_zip(get_session_cache_path(subsession_id).as_path()));
}

pub fn read_cached_session_json(subsession_id: i64) -> Value {
    return read_json_zip(get_session_cache_path(subsession_id).as_path());
}

pub fn write_cached_session_json(subsession_id: i64, json: &Value) {
    let content = json.to_string();
    write_single_file_zip(get_session_cache_path(subsession_id).as_path(), "session.json", &content);
}

pub fn write_cached_car_infos_json(json: &Value) {
    fs::write(
        get_car_data_file(),
        serde_json::to_string(&json).unwrap()
    ).unwrap();
}

pub fn write_cached_track_infos_json(json: &Value) {
    fs::write(
        get_track_data_file(),
        serde_json::to_string(&json).unwrap()
    ).unwrap();
}

pub fn get_session_cache_path(subsession_id: i64) -> PathBuf {
    return Path::new(get_sessions_dir()).join(format!("{subsession_id}.session.zip"));
}

pub fn is_session_cached(subsession_id: i64) -> bool {
    return get_session_cache_path(subsession_id).exists();
}

pub fn query_irating_history(driver_id: &DriverId, category: CategoryType) -> Value {
    let con = create_db_connection();

    let (sql, params) = Query::select()
        .column(Subsession::StartTime)
        .column(DriverResult::NewiRating)
        .column(DriverResult::NewCpi)
        .column(Session::SeriesName)
        .from(DriverResult::Table)
        .join_driver_result_to_simsession()
        .join_driver_result_to_subsession()
        .join_subsession_to_session()
        .match_driver_id(driver_id, false)
        .and_where(Expr::col(DriverResult::NewiRating).ne(-1))
        .and_where(is_event_type(EventType::Race))
        .and_where(Expr::col((Simsession::Table, Simsession::SimsessionNumber)).eq(0))
        .and_where(is_category_type(category))
        .order_by(Subsession::StartTime, Order::Asc)
        .build_rusqlite(SqliteQueryBuilder)
        ;

    let mut stmt = con.prepare(sql.as_str()).unwrap();
    let mut rows = stmt.query(&*params.as_params()).unwrap();

    let mut values = vec![];
    while let Some(row) = rows.next().unwrap() {
        let start_time: String = row.get(0).unwrap();
        let irating: i64 = row.get(1).unwrap();
        let cpi: f32 = row.get(2).unwrap();
        let series_name: String = row.get(3).unwrap();
        values.push(serde_json::json!({
            "start_time": start_time,
            "irating": irating,
            "cpi": cpi,
            "series_name": series_name,
        }));
    }

    return Value::Array(values);
}

pub struct TrackUsage {
    pub track_name: String,
    pub time: i64,
    pub laps: i64,
    pub distance: f32,
}
pub fn query_track_usage(driver_id: &DriverId) -> Vec<TrackUsage> {
    let con = create_db_connection();

    let (sql, params) = Query::select()
        .column((Track::Table, Track::TrackName))
        .select_total_time()
        .select_laps_complete()
        .select_total_distance()
        .from(DriverResult::Table)
        .join_driver_result_to_simsession()
        .join_driver_result_to_subsession()
        .join_subsession_to_session()
        .join_subsession_to_track_config()
        .join_track_config_to_track()
        .match_driver_id(driver_id, false)
        .group_by_col((Track::Table, Track::PackageId))
        .build_rusqlite(SqliteQueryBuilder);

    let mut stmt = con.prepare(sql.as_str()).unwrap();
    let mut rows = stmt.query(&*params.as_params()).unwrap();

    let mut values = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let track_name: String = row.get(0).unwrap();
        let time: i64 = row.get(1).unwrap();
        let laps: i64 = row.get(2).unwrap();
        let distance: f32 = row.get(3).unwrap();
        values.push(TrackUsage{
            track_name,
            time,
            laps,
            distance,
        });
    }

    return values;
}

pub struct CarUsage {
    pub car_name: String,
    pub time: i64,
    pub distance: f32,
}

pub fn query_car_usage(driver_id: &DriverId) -> Vec<CarUsage> {
    let con = create_db_connection();

    let (sql, params) = Query::select()
        .column((Car::Table, Car::CarName))
        .select_total_time()
        .select_total_distance()
        .from(DriverResult::Table)
        .join_driver_result_to_simsession()
        .join_driver_result_to_subsession()
        .join_driver_result_to_car()
        .join_subsession_to_session()
        .join_subsession_to_track_config()
        .join_track_config_to_track()
        .match_driver_id(driver_id, false)
        .group_by_col((Car::Table, Car::CarId))
        .build_rusqlite(SqliteQueryBuilder);

    let mut stmt = con.prepare(sql.as_str()).unwrap();
    let mut rows = stmt.query(&*params.as_params()).unwrap();

    let mut values = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let car_name: String = row.get(0).unwrap();
        let time: i64 = row.get(1).unwrap();
        let distance: f32 = row.get(2).unwrap();
        values.push(CarUsage{
            car_name,
            time,
            distance,
        });
    }

    return values;
}

pub struct CarTrackUsage {
    pub car_name: String,
    pub track_name: String,
    pub time: i64,
    pub laps: i64
}

pub fn query_track_car_usage_matrix(driver_id: &DriverId) -> Vec<CarTrackUsage> {
    let con = create_db_connection();

    let (sql, params) = Query::select()
        .column((Car::Table, Car::CarName))
        .column((Track::Table, Track::TrackName))
        .select_total_time()
        .select_laps_complete()
        .from(DriverResult::Table)
        .join_driver_result_to_simsession()
        .join_driver_result_to_subsession()
        .join_subsession_to_session()
        .join_subsession_to_track_config()
        .join_track_config_to_track()
        .join_driver_result_to_car()
        .match_driver_id(driver_id, false)
        .group_by_col((DriverResult::Table, DriverResult::CarId))
        .group_by_col((Track::Table, Track::PackageId))
        .build_rusqlite(SqliteQueryBuilder);

    let mut stmt = con.prepare(sql.as_str()).unwrap();
    let mut rows = stmt.query(&*params.as_params()).unwrap();

    let mut values = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let car_name: String = row.get(0).unwrap();
        let track_name: String = row.get(1).unwrap();
        let time: i64 = row.get(2).unwrap();
        let laps: i64 = row.get(3).unwrap();
        values.push(CarTrackUsage{
            car_name,
            track_name,
            time,
            laps,
        });
    }

    return values;
}

pub struct DriverStats {
    pub name: String,
    pub cust_id: i64,
    pub time: i64,
    pub laps: i64,
    pub distance: f32
}

pub fn query_driver_stats(driver_id: &DriverId) -> Option<DriverStats> {
    let con = create_db_connection();

    let (sql, params) = Query::select()
        .column((Driver::Table, Driver::DisplayName))
        .column((Driver::Table, Driver::CustId))
        .select_total_time()
        .select_laps_complete()
        .select_total_distance()
        .from(DriverResult::Table)
        .join_driver_result_to_simsession()
        .join_driver_result_to_subsession()
        .join_subsession_to_track_config()
        .match_driver_id(driver_id, true)
        .build_rusqlite(SqliteQueryBuilder);

    let mut stmt = con.prepare(sql.as_str()).unwrap();

    return stmt.query_row(&*params.as_params(), |row| {
        // TODO get(0) may return an error resulting in a panic
        let name: String = row.get(0)?;
        let cust_id: i64 = row.get(1)?;
        let time: i64 = row.get(2)?;
        let laps: i64 = row.get(3)?;
        let distance: f32 = row.get(4)?;
        return Ok(DriverStats {
            name, cust_id, time, laps, distance
        });
    }).ok();
}

pub fn rebuild_db_schema() {
    fs::remove_file(get_sqlite_db_file()).ok(); // ignore error

    let mut con = create_db_connection();
    let tx = con.transaction().unwrap();

    build_db_schema(&tx);
    build_db_indices(&tx);

    tx.commit().unwrap();
}

pub fn rebuild_db() {
    fs::remove_file(get_sqlite_db_file()).ok(); // ignore error

    let mut con = create_db_connection();
    con.pragma_update(None, "synchronous", "OFF").unwrap();
    con.pragma_update(None, "journal_mode", "OFF").unwrap();

    let mut tx = con.transaction().unwrap();

    build_db_schema(&tx);
    {
        let mut ctx = create_db_context(&mut tx);
        rebuild_tracks(&mut ctx);
        rebuild_cars(&mut ctx);
        rebuild_sessions(&mut ctx);
    }
    build_db_indices(&tx);

    tx.commit().unwrap();
}

pub fn update_db() {
    let mut con = create_db_connection();
    let mut tx = con.transaction().unwrap();
    {
        let mut sessions_not_in_db: Vec<i64> = Vec::new();
        {
            tx.execute("DROP TABLE IF EXISTS temp_cached_subsession_id", ()).unwrap();
            let mut stmt = tx.prepare(
                r#"SELECT temp_cached_subsession_id.subsession_id FROM temp_cached_subsession_id
                    LEFT JOIN subsession ON
                        temp_cached_subsession_id.subsession_id = subsession.subsession_id
                    WHERE
                        subsession.subsession_id IS NULL
                "#).unwrap();

            let mut rows = stmt.query(()).unwrap();


            while let Some(row) = rows.next().unwrap() {
                sessions_not_in_db.push(row.get(0).unwrap());
            }
        }

        {
            let mut ctx = crate::db::create_db_context(&mut tx);
            add_sessions_to_db(&mut ctx, sessions_not_in_db.into_iter().map(|id| get_session_cache_path(id)));
        }
    }
    tx.commit().unwrap();
}