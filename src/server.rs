use std::env;
use std::path::PathBuf;

use std::io::BufWriter;
use rocket::fs::{FileServer, Options};
use rocket::State;

use rusqlite::Connection;

use crate::driverid::DriverId;
use crate::db::{
    DbPool,
    CustomerName,
    create_r2d2_db_connection_pool,
    query_track_data,
    query_car_data,
    query_customer_names,
    query_customer_cust_ids,
    query_driver_sessions,
    query_site_team_members,
    query_team_results
};
use serde_json::{Value, json};
use crate::iracing_client::IRacingClient;

#[get("/api/v1/driver-info?<driver_name>&<cust_id>")]
async fn api_v1_driver_info(
    driver_name: Option<String>,
    cust_id: Option<i64>,
    db_pool: &State<DbPool>) -> Option<Value>
{
    if let Some(driver_id) = DriverId::from_params(driver_name, cust_id) {
        let con = db_pool.get().unwrap();
        let raw_data = query_driver_sessions(&con, &driver_id)?;

        let values: Vec<Value> = raw_data.iter().map(|data| json!({
            "subsession_id": data.subsession_id,
            "old_irating": data.old_irating,
            "new_irating": data.new_irating,
            "old_cpi": data.old_cpi,
            "new_cpi": data.new_cpi,
            "incidents": data.incidents,
            "laps_complete": data.laps_complete,
            "average_lap": data.average_lap,
            "finish_position_in_class": data.finish_position_in_class,
            "car_id": data.car_id,
            "track_id": data.track_id,
            "package_id": data.package_id,
            "license_category": data.license_category.to_db_type(),
            "start_time": data.start_time,
            "event_type": data.event_type.to_db_type(),
            "series_name": data.series_name,
            "simsession_number": data.simsession_number,
            "official_session": data.official_session,
        })).collect();

        return Some(json!({
            "sessions": values
        }));
    } else {
        return None;
    }
}

#[get("/api/v1/track-car-data")]
async fn api_v1_track_car_data(db_pool: &State<DbPool>) -> Value {
    // TODO caching
    let con = db_pool.get().unwrap();

    let track_data = query_track_data(&con);
    let car_data = query_car_data(&con);

    let mut tracks = Vec::new();
    for track in track_data {
        tracks.push(json!({
            "package_id": track.package_id,
            "track_id": track.track_id,
            "track_name": track.track_name,
            "config_name": track.config_name,
            "track_config_length": track.track_config_length,
            "corners_per_lap": track.corners_per_lap,
            "category": track.category.to_db_type()
        }));
    }

    let mut cars = Vec::new();
    for car in car_data {
        cars.push(json!({
            "car_id": car.car_id,
            "car_name": car.car_name,
            "car_name_abbreviated": car.car_name_abbreviated
        }));
    }

    return json!({
        "tracks": tracks,
        "cars": cars
    });
}

fn parse_team_customer_infos(con: &Connection, team: &String) -> Vec<CustomerName> {
    return query_site_team_members(con, team);
}

fn parse_drivers_customer_infos(drivers: &String) -> Option<Vec<CustomerName>> {
    let mut infos = Vec::new();
    for driver in drivers.split(";") {
        if driver.is_empty() {
            continue;
        }

        if driver.chars().nth(0).unwrap() == '$' {
            let cust_id_str: String = driver.chars().skip(1).collect();
            if let Ok(cust_id) = cust_id_str.parse::<i64>() {
                infos.push(CustomerName{name: "".to_owned(), cust_id})
            } else {
                return None;
            }
        } else {
            infos.push(CustomerName{name: driver.to_owned(), cust_id: -1})
        }
    }
    return Some(infos);
}

#[get("/api/v1/customers?<team>&<drivers>")]
async fn api_v1_customers(
    team: Option<String>,
    drivers: Option<String>,
    db_pool: &State<DbPool>) -> Option<Value>
{
    let con = db_pool.get().unwrap();

    let infos;
    if let Some(team) = team {
        infos = parse_team_customer_infos(&con, &team);
    } else if let Some(drivers) = drivers {
        infos = parse_drivers_customer_infos(&drivers)?;
    } else {
        return Option::None;
    }

    // fill out missing info
    let mut cust_ids = Vec::new();
    let mut names = Vec::new();
    let mut result = Vec::new();
    for info in infos.into_iter() {
        if info.name.is_empty() {
            cust_ids.push(info.cust_id);
        } else if info.cust_id == -1 {
            names.push(info.name);
        } else {
            result.push(info);
        }
    }

    result.append(&mut query_customer_names(&con, cust_ids));
    result.append(&mut query_customer_cust_ids(&con, names));

    let json_arr = result.iter().map(|name| {
        return json!({
            "name": name.name,
            "cust_id": name.cust_id
        });
    }).collect();

    return Some(Value::Array(json_arr));
}

fn semi_colon_string_to_i64s(ids: &String) -> Vec<i64> {
    let id_strs = ids.split(";");
    let mut id_nums = vec![];
    for str in id_strs {
        if let Ok(num) = str.parse::<i64>() {
            id_nums.push(num);
        }
    }
    return id_nums;
}

#[get("/api/v1/customer-names?<cust_ids>")]
async fn api_v1_customer_names(
    cust_ids: String,
    db_pool: &State<DbPool>) -> Value
{
    let cust_id_nums = semi_colon_string_to_i64s(&cust_ids);

    let con = db_pool.get().unwrap();
    let names = query_customer_names(&con, cust_id_nums);

    let result = names.iter().map(|name| {
        return json!({
            "name": name.name,
            "cust_id": name.cust_id
        });
    }).collect();

    return Value::Array(result);
}

enum TeamResultsFormat {
    Json,
    Csv
}

#[get("/api/v1/team-results-csv?<team_ids>")]
async fn api_v1_team_results_csv(
    team_ids: String,
    db_pool: &State<DbPool>) -> String
{
    let team_ids = semi_colon_string_to_i64s(&team_ids);

    let con = db_pool.get().unwrap();

    let raw_data = query_team_results(&con, team_ids);

    let mut writer = csv::Writer::from_writer(Vec::new());

    // header
    writer.write_record(&[
        "subsession_id",
        "cust_id",
        "team_id",
        "driver_name",
        "track_id",
        "package_id",
        "car_id",
        "laps_complete",
        "finish_position_in_class",
        "incidents",
        "start_time"
    ]).unwrap();

    // values
    raw_data.iter().for_each(|data| {
        writer.write_record(&[
            data.subsession_id.to_string(),
            data.cust_id.to_string(),
            data.team_id.to_string(),
            data.driver_name.to_string(),
            data.track_id.to_string(),
            data.package_id.to_string(),
            data.car_id.to_string(),
            data.laps_complete.to_string(),
            data.finish_position_in_class.to_string(),
            data.incidents.to_string(),
            data.start_time.to_string()
        ]).unwrap();
    });

    return String::from_utf8(writer.into_inner().unwrap()).unwrap();
}

#[get("/api/v1/team-results?<team_ids>")]
async fn api_v1_team_results(
    team_ids: String,
    db_pool: &State<DbPool>) -> Value
{
    let team_ids = semi_colon_string_to_i64s(&team_ids);

    let con = db_pool.get().unwrap();

    let raw_data = query_team_results(&con, team_ids);

    let values: Vec<Value> = raw_data.iter().map(|data| json!({
        "subsession_id": data.subsession_id,
        "cust_id": data.cust_id,
        "team_id": data.team_id,
        "driver_name": data.driver_name,
        "track_id": data.track_id,
        "package_id": data.package_id,
        "car_id": data.car_id,
        "laps_complete": data.laps_complete,
        "finish_position_in_class": data.finish_position_in_class,
        "incidents": data.incidents,
        "start_time": data.start_time,
    })).collect();

    return json!({
        "results": values
    });
}

pub async fn start_rocket_server(enable_https: bool) {
    const SITE_DIR_ENV_VAR: &str = "IRACING_STATS_SITE_DIR";
    const LOG_FILE_ENV_VAR: &str = "IRACING_STATS_LOG_FILE";

    let db_pool = create_r2d2_db_connection_pool();

    let mut figment = rocket::Config::figment();
    if enable_https {
        figment = figment
            .merge(("tls.certs", "path/to/certs.pem"))
            .merge(("tls.key", vec![0; 32]));
    }

    let site_dir = match env::var(SITE_DIR_ENV_VAR) {
        Ok(value) => value,
        Err(_error) => "site/build".to_owned()
    };

    let log_file = match env::var(LOG_FILE_ENV_VAR) {
        Ok(value) => value,
        Err(_error) => "server.log".to_owned()
    };
    let server_logger = crate::server_logger::ServerLogger::new(PathBuf::from(log_file));

    let _result = rocket::custom(figment)
        // .mount("/static", FileServer::from("static"))
        .mount("/iracing-stats", FileServer::new(site_dir, Options::Index))
        .mount("/", routes![
            api_v1_customers,
            api_v1_customer_names,
            api_v1_driver_info,
            api_v1_track_car_data,
            api_v1_team_results,
            api_v1_team_results_csv,
        ])
        .manage(IRacingClient::new())
        .manage(db_pool)
        .attach(server_logger)
        .launch().await.unwrap();
}