use std::fs;
use serde_json;
use sqlite;

// const SESSIONS_DIR: &str = "data/sessions";
// const TRACK_DATA_FILE: &str = "data/tracks.json";
// const CAR_DATA_FILE: &str = "data/cars.json";
const SQLITE_DB_FILE: &str = "stats.db";
// const BASEURL: &str = "https://members-ng.iracing.com";

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

fn main() {
    let con = sqlite::open(SQLITE_DB_FILE).unwrap();
    let mut statement = con.prepare("SELECT * FROM track").unwrap();
    while let Ok(sqlite::State::Row) = statement.next() {
        println!("id = {}, name = {}",
            statement.read::<i64, _>("package_id").unwrap(),
            statement.read::<String, _>("track_name").unwrap()
        );
    }
    parse_session_zip("data/sessions/10004652.session.zip");
}
