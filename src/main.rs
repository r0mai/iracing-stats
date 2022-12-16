use std::fs;
use serde_json;

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
    parse_session_zip("data/sessions/10004652.session.zip");
}
