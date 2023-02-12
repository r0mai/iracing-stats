use std::fs::{File, read_to_string};
use simple_xml_builder::XMLElement;
use serde_json::Value;

use crate::db::get_track_data_file;

fn add_constant(element: &mut XMLElement, name: &str, value: &String, unit: &str) {
    let mut corners = XMLElement::new("MathConstant");
    corners.add_attribute("Name", name);
    corners.add_attribute("Value", value);
    corners.add_attribute("Unit", unit);
    element.add_child(corners);
}

pub fn output_motec_track_xmls() {
    let mut xml_file = File::create("tracks.xml").unwrap();

    let track_contents = read_to_string(get_track_data_file()).unwrap();
    let tracks: Value = serde_json::from_str(&track_contents).unwrap();

    let mut root = XMLElement::new("Root");

    for track in tracks.as_array().unwrap() {
        let mut maths = XMLElement::new("Maths");
        // maths.add_attribute("Locale", "Hungarian_Hungary.1250");
        // maths.add_attribute("DefaultLocale", "C");
        maths.add_attribute("TrackName", track["track_name"].as_str().unwrap());
        maths.add_attribute("ConfigName", track["config_name"].as_str().unwrap_or(""));

        let mut constants = XMLElement::new("MathConstants");

        add_constant(&mut constants, "corners", &track["corners_per_lap"].as_i64().unwrap().to_string(), "");
        add_constant(&mut constants, "pit speed", &track["pit_road_speed_limit"].as_i64().unwrap_or(0).to_string(), "km/h");
        add_constant(&mut constants, "track_id", &track["track_id"].as_i64().unwrap().to_string(), "");
        add_constant(&mut constants, "track_length", &track["track_config_length"].as_f64().unwrap().to_string(), "km");

        maths.add_child(constants);
        root.add_child(maths);
    }

    root.write(xml_file).unwrap();
}