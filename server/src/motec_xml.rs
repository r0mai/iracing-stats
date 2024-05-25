use std::fs::{File, read_to_string};
use simple_xml_builder::XMLElement;
use serde_json::Value;

use crate::db::{get_track_data_file, get_car_data_file};

fn add_constant(element: &mut XMLElement, name: &str, value: &String, unit: &str) {
    let mut corners = XMLElement::new("MathConstant");
    corners.add_attribute("Name", name);
    corners.add_attribute("Value", value);
    corners.add_attribute("Unit", unit);
    element.add_child(corners);
}

pub fn output_motec_track_xmls() {

    let track_contents = read_to_string(get_track_data_file()).unwrap();
    let tracks: Value = serde_json::from_str(&track_contents).unwrap();

    for track in tracks.as_array().unwrap() {
        let dir_path = track["track_dirpath"].as_str().unwrap().replace('\\', " ");
        let id = format!("track {}", dir_path);
        let xml_name = format!("motec/{}.xml", id);

        let xml_file = File::create(xml_name).unwrap();

        let mut maths = XMLElement::new("Maths");
        maths.add_attribute("Locale", "Hungarian_Hungary.1250");
        maths.add_attribute("DefaultLocale", "C");
        maths.add_attribute("Id", id);
        maths.add_attribute("Condition", format!("'Venue' == \"{}\"", dir_path));

        let mut constants = XMLElement::new("MathConstants");

        add_constant(&mut constants, "ai_enabled", &track["ai_enabled"].as_bool().unwrap().to_string(), "");
        add_constant(&mut constants, "config_name", &track["config_name"].as_str().unwrap_or("").to_string(), "");
        add_constant(&mut constants, "corners_per_lap", &track["corners_per_lap"].as_i64().unwrap().to_string(), "");
        add_constant(&mut constants, "grid_stalls", &track["grid_stalls"].as_i64().unwrap_or(0).to_string(), "");
        add_constant(&mut constants, "number_pitstalls", &track["number_pitstalls"].as_i64().unwrap_or(0).to_string(), "");
        add_constant(&mut constants, "pit_road_speed_limit", &track["pit_road_speed_limit"].as_i64().unwrap_or(0).to_string(), "mph");
        add_constant(&mut constants, "track_config_length", &track["track_config_length"].as_f64().unwrap().to_string(), "mile");
        add_constant(&mut constants, "track_id", &track["track_id"].as_i64().unwrap().to_string(), "");
        add_constant(&mut constants, "track_name", &track["track_name"].as_str().unwrap().to_string(), "");

        maths.add_child(constants);
        maths.write(xml_file).unwrap();
    }

}

pub fn add_math_enum<F, G>(root: &mut XMLElement, arr: &Vec<Value>, enum_name: &str, name_func: F, value_func: &G) where
    F: Fn(&Value) -> String, G: Fn(&Value) -> String
{
    let mut math_enum = XMLElement::new("MathEnumeration");
    math_enum.add_attribute("Name", enum_name);

    for element in arr {
        let mut pair = XMLElement::new("Enum");
        pair.add_attribute("Name", name_func(&element));
        pair.add_attribute("Value", value_func(&element));

        math_enum.add_child(pair);
    }

    root.add_child(math_enum);
}

pub fn add_math_enum_str(root: &mut XMLElement, arr: &Vec<Value>, id_str: &str, enum_name: &str) {
    let track_id_f = |v: &Value| v[id_str].to_string();
    add_math_enum(root, arr, enum_name, |v| v[enum_name].to_string(), &track_id_f);
}

pub fn add_choose_expr<F>(root: &mut XMLElement, arr: &Vec<Value>, enum_name: &str, expr_func: F) where
    F: Fn(&Value) -> String
{
    let mut math_expr = XMLElement::new("MathExpression");

    math_expr.add_attribute("Id", enum_name);
    math_expr.add_attribute("DisplayDPS", "0");
    math_expr.add_attribute("DisplayColorIndex", "2");
    math_expr.add_attribute("Interpolate", "0");
    math_expr.add_attribute("EnumerationName", "");

    let mut prefix = String::new();
    let mut suffix = "invalid()".to_string();
    for element in arr {
        prefix += format!("choose({},", expr_func(element)).as_str();
        suffix += ")";
    }

    math_expr.add_attribute("Script", prefix + suffix.as_str());

    root.add_child(math_expr);
}

pub fn output_motec_car_xmls2() {
    let track_xml = "motec/car_data.xml";

    let track_contents = read_to_string(get_car_data_file()).unwrap();
    let tracks: Value = serde_json::from_str(&track_contents).unwrap();

    let xml_file = File::create(track_xml).unwrap();

    let mut maths = XMLElement::new("Maths");
    maths.add_attribute("Locale", "Hungarian_Hungary.1250");
    maths.add_attribute("DefaultLocale", "C");
    maths.add_attribute("Id", "car 101");

    let mut math_enums = XMLElement::new("MathEnumerations");

    let track_arr = tracks.as_array().unwrap();

    add_math_enum_str(&mut math_enums, &track_arr, "car_id", "car_name");

    let mut math_items = XMLElement::new("MathItems");

    add_choose_expr(&mut math_items, &track_arr, "car ID list",
        |e: &Value| format!("'Venue'==\"{}\",{}", e["car_dirpath"].to_string(), e["car_id"].to_string())
    );

    maths.add_child(math_enums);
    maths.add_child(math_items);

    maths.write(xml_file).unwrap();
}

pub fn output_motec_track_xmls2() {
    let track_xml = "motec/track_data.xml";

    let track_contents = read_to_string(get_track_data_file()).unwrap();
    let tracks: Value = serde_json::from_str(&track_contents).unwrap();

    let xml_file = File::create(track_xml).unwrap();

    let mut maths = XMLElement::new("Maths");
    maths.add_attribute("Locale", "Hungarian_Hungary.1250");
    maths.add_attribute("DefaultLocale", "C");
    maths.add_attribute("Id", "track 101");

    let mut math_enums = XMLElement::new("MathEnumerations");

    let track_arr = tracks.as_array().unwrap();

    add_math_enum_str(&mut math_enums, &track_arr, "track_id", "config_name");
    add_math_enum_str(&mut math_enums, &track_arr, "track_id", "ai_enabled");
    add_math_enum_str(&mut math_enums, &track_arr, "track_id", "corners_per_lap");
    add_math_enum_str(&mut math_enums, &track_arr, "track_id", "grid_stalls");
    add_math_enum_str(&mut math_enums, &track_arr, "track_id", "number_pitstalls");
    add_math_enum_str(&mut math_enums, &track_arr, "track_id", "pit_road_speed_limit");
    add_math_enum_str(&mut math_enums, &track_arr, "track_id", "track_config_length");
    add_math_enum_str(&mut math_enums, &track_arr, "track_id", "track_id");
    add_math_enum_str(&mut math_enums, &track_arr, "track_id", "track_name");

    let mut math_items = XMLElement::new("MathItems");

    add_choose_expr(&mut math_items, &track_arr, "track ID list",
        |e: &Value| format!("'Venue'==\"{}\",{}", e["track_dirpath"].to_string(), e["track_id"].to_string())
    );

    maths.add_child(math_enums);
    maths.add_child(math_items);

    maths.write(xml_file).unwrap();
}