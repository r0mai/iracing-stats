use std::{fs, path::PathBuf, path::Path, io::Write, env};
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::{self, Value};
use rusqlite;
use rusqlite::Connection;
use chrono::{self, TimeZone};
use zip::write::FileOptions;
use lazy_static::lazy_static;
use sea_query_rusqlite::RusqliteBinder;
use sea_query::{
    Query,
    Expr,
    Order,
    SqliteQueryBuilder,
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
    SiteTeam,
    SchemaUtils,
    SiteTeamMember,
    is_event_type,
    is_main_event
};
use crate::event_type::EventType;
use crate::category_type::CategoryType;
use crate::driverid::DriverId;

const SESSIONS_DIR: &str = "data/sessions";
const TRACK_DATA_FILE: &str = "data/tracks.json";
const CAR_DATA_FILE: &str = "data/cars.json";
const SEASON_DATA_FILE: &str = "data/seasons.json";
const SITE_TEAMS_DATA_FILE: &str = "static-data/site-teams.json";
const SQLITE_DB_FILE: &str = "stats.db";
const BASE_DIR_ENV_VAR: &str = "IRACING_STATS_BASE_DIR";
const STATIC_DIR_ENV_VAR: &str = "IRACING_STATS_STATIC_DIR";

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

fn get_static_dir() -> &'static Path {
    lazy_static! {
        static ref STATIC_DIR: PathBuf = PathBuf::from(
            match env::var(STATIC_DIR_ENV_VAR) {
                Ok(value) => value,
                Err(_error) => ".".to_owned()
            }
        );
    }
    return STATIC_DIR.as_path();
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

pub fn get_season_data_file() -> &'static Path {
    lazy_static! {
        static ref FILE: PathBuf = get_base_dir().join(SEASON_DATA_FILE);
    }
    return FILE.as_path();
}

pub fn get_sessions_dir() -> &'static Path {
    lazy_static! {
        static ref DIR: PathBuf = get_base_dir().join(SESSIONS_DIR);
    }
    return DIR.as_path();
}

pub fn get_site_teams_data_file() -> &'static Path {
    lazy_static! {
        static ref FILE: PathBuf = get_static_dir().join(SITE_TEAMS_DATA_FILE);
    }
    return FILE.as_path();
}

pub fn create_db_connection() -> rusqlite::Connection {
    return rusqlite::Connection::open(get_sqlite_db_file()).unwrap();
}

pub type DbPool = r2d2::Pool<SqliteConnectionManager>;

pub fn create_r2d2_db_connection_pool() -> DbPool {
    let manager = SqliteConnectionManager::file(get_sqlite_db_file());
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    return pool;
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
    insert_season_statement: rusqlite::Statement<'a>,
    insert_site_team_statement: rusqlite::Statement<'a>,
    insert_site_team_member_statement: rusqlite::Statement<'a>,
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
            ?, /* track_config_length */
            ?, /* corners_per_lap */
            ?  /* category_id */
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
            ?, /* track_id */
            ?  /* official_session */
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
    let insert_driver_statement = tx.prepare(r#"
        INSERT OR IGNORE INTO driver VALUES(
            ?, /* cust_id */
            ?  /* display_name */
    );"#).unwrap();
    let insert_driver_result_statement = tx.prepare(r#"
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
    let insert_season_statement = tx.prepare(r#"
        INSERT INTO season VALUES(
            ?, /* season_id */
            ?, /* series_id */
            ?, /* season_name */
            ?, /* series_name */
            ?, /* official */
            ?, /* season_year */
            ?, /* season_quarter */
            ?, /* license_group_id */
            ?, /* fixed_setup */
            ?  /* driver_changes */
    );"#).unwrap();
    let insert_site_team_statement = tx.prepare(r#"
        INSERT INTO site_team VALUES(
            ?, /* site_team_id */
            ?  /* site_team_name */
    );"#).unwrap();
    let insert_site_team_member_statement = tx.prepare(r#"
        INSERT INTO site_team_member VALUES(
            ?, /* site_team_id */
            ?  /* cust_id */
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
        insert_season_statement,
        insert_site_team_statement,
        insert_site_team_member_statement,
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

fn miles_to_km(miles: f64) -> f64 {
    return miles * 1.60934;
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
        miles_to_km(track["track_config_length"].as_f64().unwrap()),
        track["corners_per_lap"].as_i64().unwrap(),
        track["category_id"].as_i64().unwrap()
    )).unwrap();
}

fn add_car_to_db(ctx: &mut DbContext, car: &Value) {
    ctx.insert_car_statement.execute((
        car["car_id"].as_i64().unwrap(),
        car["car_name"].as_str().unwrap(),
        car["car_name_abbreviated"].as_str().unwrap(),
    )).unwrap();
}

fn add_season_to_db(ctx: &mut DbContext, season: &Value) {
    ctx.insert_season_statement.execute((
        season["season_id"].as_i64().unwrap(),
        season["series_id"].as_i64().unwrap(),
        season["season_name"].as_str().unwrap(),
        season["series_name"].as_str().unwrap(),
        season["official"].as_bool().unwrap(),
        season["season_year"].as_i64().unwrap(),
        season["season_quarter"].as_i64().unwrap(),
        season["license_group"].as_i64().unwrap(),
        season["fixed_setup"].as_bool().unwrap(),
        season["driver_changes"].as_bool().unwrap(),
    )).unwrap();
}

fn add_site_team_to_db(ctx: &mut DbContext, id: usize, team: &Value) {
    ctx.insert_site_team_statement.execute((
        id,
        team["name"].as_str().unwrap()
    )).unwrap();

    for member in team["members"].as_array().unwrap() {
        ctx.insert_site_team_member_statement.execute((
            id,
            member["cust_id"].as_i64().unwrap()
        )).unwrap();
    }
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
        subsession["track"]["track_id"].as_i64().unwrap(),
        subsession["official_session"].as_bool().unwrap(),
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

pub fn rebuild_tracks(ctx: &mut DbContext) {
    let contents = fs::read_to_string(get_track_data_file()).unwrap();
    let tracks: Value = serde_json::from_str(&contents).unwrap();

    for track in tracks.as_array().unwrap() {
        add_track_to_db(ctx, &track);
    }
}

pub fn rebuild_cars(ctx: &mut DbContext) {
    let contents = fs::read_to_string(get_car_data_file()).unwrap();
    let cars: Value = serde_json::from_str(&contents).unwrap();

    for car in cars.as_array().unwrap() {
        add_car_to_db(ctx, &car);
    }
}

pub fn rebuild_seasons(ctx: &mut DbContext) {
    let contents = fs::read_to_string(get_season_data_file()).unwrap();
    let seasons: Value = serde_json::from_str(&contents).unwrap();

    for season in seasons.as_array().unwrap() {
        // This is a duplicated season_id :/
        if season["season_id"].as_i64().unwrap() == 4222 && season["season_year"].as_i64().unwrap() == 2023 {
            return;
        }
        add_season_to_db(ctx, &season);
    }
}

fn rebuild_sessions(ctx: &mut DbContext) {
    let paths = fs::read_dir(get_sessions_dir()).unwrap();
    add_sessions_to_db(ctx, paths.map(|e| e.unwrap().path()));
}

fn rebuild_site_teams(ctx: &mut DbContext) {
    let contents = fs::read_to_string(get_site_teams_data_file()).unwrap();
    let teams: Value = serde_json::from_str(&contents).unwrap();

    for (id, team) in teams.as_array().unwrap().into_iter().enumerate() {
        add_site_team_to_db(ctx, id, team);
    }
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

pub fn write_cached_seasons_json(json: &Value) {
    fs::write(
        get_season_data_file(),
        serde_json::to_string(&json).unwrap()
    ).unwrap();
}

pub fn get_session_cache_path(subsession_id: i64) -> PathBuf {
    return Path::new(get_sessions_dir()).join(format!("{subsession_id}.session.zip"));
}

pub fn is_session_cached(subsession_id: i64) -> bool {
    return get_session_cache_path(subsession_id).exists();
}

pub struct DriverSession {
    pub subsession_id: i64,
    pub old_irating: i32,
    pub new_irating: i32,
    pub old_cpi: f32,
    pub new_cpi: f32,
    pub incidents: i32,
    pub laps_complete: i32,
    pub average_lap: i64,
    pub finish_position_in_class: i32,
    pub car_id: i32,
    pub track_id: i32,
    pub package_id: i32,
    pub license_category: CategoryType,
    pub start_time: String,
    pub event_type: EventType,
    pub series_name: String,
    pub simsession_number: i32,
    pub simsession_type: i32,
    pub official_session: bool,
}

pub fn query_driver_sessions(con: &Connection, driver_id: &DriverId) -> Option<Vec<DriverSession>> {
    let (sql, params) = Query::select()
        .column((DriverResult::Table, DriverResult::SubsessionId))
        .column((DriverResult::Table, DriverResult::OldiRating))
        .column((DriverResult::Table, DriverResult::NewiRating))
        .column((DriverResult::Table, DriverResult::OldCpi))
        .column((DriverResult::Table, DriverResult::NewCpi))
        .column((DriverResult::Table, DriverResult::Incidents))
        .column((DriverResult::Table, DriverResult::LapsComplete))
        .column((DriverResult::Table, DriverResult::AverageLap))
        .column((DriverResult::Table, DriverResult::FinishPositionInClass))
        .column((DriverResult::Table, DriverResult::CarId))
        .column((TrackConfig::Table, TrackConfig::TrackId))
        .column((TrackConfig::Table, TrackConfig::PackageId))
        .column((Subsession::Table, Subsession::LicenseCategoryId))
        .column((Subsession::Table, Subsession::StartTime))
        .column((Subsession::Table, Subsession::EventType))
        .column((Session::Table, Session::SeriesName))
        .column((Simsession::Table, Simsession::SimsessionNumber))
        .column((Simsession::Table, Simsession::SimsessionType))
        .column((Subsession::Table, Subsession::OfficialSession))
        .from(DriverResult::Table)
        .join_driver_result_to_subsession()
        .join_driver_result_to_simsession()
        .join_subsession_to_session()
        .join_subsession_to_track_config()
        .match_driver_id(driver_id, false)
        .build_rusqlite(SqliteQueryBuilder);

    let mut stmt = con.prepare(sql.as_str()).unwrap();
    let mut rows = stmt.query(&*params.as_params()).unwrap();

    let mut values = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        values.push(DriverSession{
            subsession_id: row.get(0).unwrap(),
            old_irating: row.get(1).unwrap(),
            new_irating: row.get(2).unwrap(),
            old_cpi: row.get(3).unwrap(),
            new_cpi: row.get(4).unwrap(),
            incidents: row.get(5).unwrap(),
            laps_complete: row.get(6).unwrap(),
            average_lap: row.get(7).unwrap(),
            finish_position_in_class: row.get(8).unwrap(),
            car_id: row.get(9).unwrap(),
            track_id: row.get(10).unwrap(),
            package_id: row.get(11).unwrap(),
            license_category: CategoryType::from_i32(row.get(12).unwrap()).ok()?,
            start_time: row.get(13).unwrap(),
            event_type: EventType::from_i32(row.get(14).unwrap()).ok()?,
            series_name: row.get(15).unwrap(),
            simsession_number: row.get(16).unwrap(),
            simsession_type: row.get(17).unwrap(),
            official_session: row.get(18).unwrap(),
        });
    }

    return Some(values);
}

pub struct TeamResult {
    pub subsession_id: i64,
    pub cust_id: i64,
    pub team_id: i64,
    pub driver_name: String,
    pub track_id: i32,
    pub package_id: i32,
    pub car_id: i32,
    // pub car_name: String,
    pub laps_complete: i32,
    pub finish_position_in_class: i32,
    pub incidents: i32,
    pub start_time: String,
}

pub fn query_team_results(con: &Connection, team_ids: Vec<i64>) -> Vec<TeamResult> {
    let (sql, params) = Query::select()
        .column((DriverResult::Table, DriverResult::SubsessionId))
        .column((DriverResult::Table, DriverResult::CustId))
        .column((DriverResult::Table, DriverResult::TeamId))
        .column((Driver::Table, Driver::DisplayName))
        .column((TrackConfig::Table, TrackConfig::TrackId))
        .column((TrackConfig::Table, TrackConfig::PackageId))
        .column((DriverResult::Table, DriverResult::CarId))
        .column((DriverResult::Table, DriverResult::LapsComplete))
        .column((DriverResult::Table, DriverResult::FinishPositionInClass))
        .column((DriverResult::Table, DriverResult::Incidents))
        .column((Subsession::Table, Subsession::StartTime))
        .from(DriverResult::Table)
        .join_driver_result_to_subsession()
        .join_driver_result_to_simsession()
        .join_driver_result_to_driver()
        .join_subsession_to_session()
        .join_subsession_to_track_config()
        .and_where(Expr::col((DriverResult::Table, DriverResult::TeamId)).is_in(team_ids))
        .and_where(is_event_type(EventType::Race))
        .and_where(is_main_event())
        .order_by((Subsession::Table, Subsession::StartTime), Order::Asc)
        .build_rusqlite(SqliteQueryBuilder);

    let mut stmt = con.prepare(sql.as_str()).unwrap();
    let mut rows = stmt.query(&*params.as_params()).unwrap();

    let mut values = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        values.push(TeamResult{
            subsession_id: row.get(0).unwrap(),
            cust_id: row.get(1).unwrap(),
            team_id: row.get(2).unwrap(),
            driver_name: row.get(3).unwrap(),
            track_id: row.get(4).unwrap(),
            package_id: row.get(5).unwrap(),
            car_id: row.get(6).unwrap(),
            laps_complete: row.get(7).unwrap(),
            finish_position_in_class: row.get(8).unwrap(),
            incidents: row.get(9).unwrap(),
            start_time: row.get(10).unwrap()
        });
    }

    return values;
}

pub struct CustomerName {
    pub cust_id: i64,
    pub name: String
}

pub fn query_customer_cust_ids(con: &Connection, names: Vec<String>) -> Vec<CustomerName> {
    let (sql, params) = Query::select()
        .column((Driver::Table, Driver::DisplayName))
        .column((Driver::Table, Driver::CustId))
        .from(Driver::Table)
        .and_where(Expr::col(Driver::DisplayName).is_in(names))
        .build_rusqlite(SqliteQueryBuilder);

    let mut stmt = con.prepare(sql.as_str()).unwrap();
    let mut rows = stmt.query(&*params.as_params()).unwrap();

    let mut values = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let name: String = row.get(0).unwrap();
        let cust_id: i64 = row.get(1).unwrap();

        values.push(CustomerName{
            cust_id, name
        });
    }

    return values;
}

pub fn query_customer_names(con: &Connection, cust_ids: Vec<i64>) -> Vec<CustomerName> {
    let (sql, params) = Query::select()
        .column((Driver::Table, Driver::DisplayName))
        .column((Driver::Table, Driver::CustId))
        .from(Driver::Table)
        .and_where(Expr::col(Driver::CustId).is_in(cust_ids))
        .build_rusqlite(SqliteQueryBuilder);

    let mut stmt = con.prepare(sql.as_str()).unwrap();
    let mut rows = stmt.query(&*params.as_params()).unwrap();

    let mut values = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let name: String = row.get(0).unwrap();
        let cust_id: i64 = row.get(1).unwrap();

        values.push(CustomerName{
            cust_id, name
        });
    }

    return values;
}

pub fn query_site_team_members(con: &Connection, team: &String) -> Vec<CustomerName> {
    let (sql, params) = Query::select()
        .column((Driver::Table, Driver::DisplayName))
        .column((Driver::Table, Driver::CustId))
        .from(SiteTeam::Table)
        .join_site_team_to_site_team_member()
        .join_site_team_member_to_driver()
        .and_where(Expr::col(SiteTeam::SiteTeamName).eq(team))
        .build_rusqlite(SqliteQueryBuilder);

    // select display_name, site_team_member.cust_id
    // from site_team
    // join site_team_member on site_team.site_team_id == site_team_member.site_team_id
    // join driver on driver.cust_id == site_team_member.cust_id
    // where site_team_name == 'rsmr';

    let mut stmt = con.prepare(sql.as_str()).unwrap();
    let mut rows = stmt.query(&*params.as_params()).unwrap();

    let mut values = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let name: String = row.get(0).unwrap();
        let cust_id: i64 = row.get(1).unwrap();

        values.push(CustomerName{
            cust_id, name
        });
    }

    return values;
}

pub struct CarData {
    pub car_id: i64,
    pub car_name: String,
    pub car_name_abbreviated: String
}

pub fn query_car_data(con: &Connection) -> Vec<CarData> {
    let (sql, params) = Query::select()
        .column((Car::Table, Car::CarId))
        .column((Car::Table, Car::CarName))
        .column((Car::Table, Car::CarNameAbbreviated))
        .from(Car::Table)
        .build_rusqlite(SqliteQueryBuilder);

    let mut stmt = con.prepare(sql.as_str()).unwrap();
    let mut rows = stmt.query(&*params.as_params()).unwrap();

    let mut values = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let car_id: i64 = row.get(0).unwrap();
        let car_name: String = row.get(1).unwrap();
        let car_name_abbreviated = row.get(2).unwrap();

        values.push(CarData{
            car_id,
            car_name,
            car_name_abbreviated
        });
    }

    return values;
}

pub struct TrackData {
    pub package_id: i64,
    pub track_id: i64,
    pub track_name: String,
    pub config_name: String,
    pub track_config_length: f32,
    pub corners_per_lap: i32,
    pub category: CategoryType,
}

pub fn query_track_data(con: &Connection) -> Vec<TrackData> {
    let (sql, params) = Query::select()
        .column((Track::Table, Track::PackageId))
        .column((TrackConfig::Table, TrackConfig::TrackId))
        .column((Track::Table, Track::TrackName))
        .column((TrackConfig::Table, TrackConfig::ConfigName))
        .column((TrackConfig::Table, TrackConfig::TrackConfigLength))
        .column((TrackConfig::Table, TrackConfig::CornersPerLap))
        .column((TrackConfig::Table, TrackConfig::CategoryId))
        .from(TrackConfig::Table)
        .join_track_config_to_track()
        .build_rusqlite(SqliteQueryBuilder);

    let mut stmt = con.prepare(sql.as_str()).unwrap();
    let mut rows = stmt.query(&*params.as_params()).unwrap();

    let mut values = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let package_id: i64 = row.get(0).unwrap();
        let track_id: i64 = row.get(1).unwrap();
        let track_name: String = row.get(2).unwrap();
        let config_name: String = row.get(3).unwrap();
        let track_config_length: f32 = row.get(4).unwrap();
        let corners_per_lap: i32 = row.get(5).unwrap();
        let category = CategoryType::from_i32(row.get(6).unwrap()).unwrap();

        values.push(TrackData{
            package_id,
            track_id,
            track_name,
            config_name,
            track_config_length,
            corners_per_lap,
            category
        });
    }

    return values;
}

pub fn query_all_site_team_members(con: &Connection) -> Vec<i64> {
    let (sql, params) = Query::select()
        .distinct()
        .column((SiteTeamMember::Table, SiteTeamMember::CustId))
        .from(SiteTeamMember::Table)
        .build_rusqlite(SqliteQueryBuilder);

    let mut stmt = con.prepare(sql.as_str()).unwrap();
    let mut rows = stmt.query(&*params.as_params()).unwrap();

    let mut cust_ids = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        cust_ids.push(row.get(0).unwrap());
    }
    return cust_ids;
}

pub fn rebuild_db_schema() {
    fs::remove_file(get_sqlite_db_file()).ok(); // ignore error

    let mut con = create_db_connection();
    let tx = con.transaction().unwrap();

    build_db_schema(&tx);
    build_db_indices(&tx);

    tx.commit().unwrap();
}

pub fn rebuild_tracks_in_db() {
    let mut con = create_db_connection();
    let mut tx = con.transaction().unwrap();
    {
        tx.execute("DELETE FROM track", ()).unwrap();
        tx.execute("DELETE FROM track_config", ()).unwrap();

        let mut ctx = create_db_context(&mut tx);
        rebuild_tracks(&mut ctx);
    }
    tx.commit().unwrap();
}

pub fn rebuild_cars_in_db() {
    let mut con = create_db_connection();
    let mut tx = con.transaction().unwrap();
    {
        tx.execute("DELETE FROM car", ()).unwrap(); // deletes all rows

        let mut ctx = create_db_context(&mut tx);
        rebuild_cars(&mut ctx);
    }
    tx.commit().unwrap();
}

pub fn rebuild_seasons_in_db() {
    let mut con = create_db_connection();
    let mut tx = con.transaction().unwrap();
    {
        tx.execute("DELETE FROM season", ()).unwrap(); // deletes all rows

        let mut ctx = create_db_context(&mut tx);
        rebuild_seasons(&mut ctx);
    }
    tx.commit().unwrap();
}

pub fn rebuild_site_teams_in_db() {
    let mut con = create_db_connection();
    let mut tx = con.transaction().unwrap();
    {
        tx.execute("DELETE FROM site_team", ()).unwrap(); // deletes all rows
        tx.execute("DELETE FROM site_team_member", ()).unwrap(); // deletes all rows

        let mut ctx = create_db_context(&mut tx);
        rebuild_site_teams(&mut ctx);
    }
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
        rebuild_seasons(&mut ctx);
        rebuild_site_teams(&mut ctx);
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