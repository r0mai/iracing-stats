use std::collections::HashMap;
use itertools::Itertools;

use crate::{category_type::CategoryType, db::{create_db_connection, query_discord_report, DiscordRaceResultReport, DiscordTeamRaceResultReport}, event_type::EventType};

fn finish_reason_string(reason_out: &String) -> String {
    if reason_out == "Running" {
        return "".to_string();
    }

    if reason_out == "" {
        return " (Unknown out reason)".to_string();
    }

    return format!(" ({})", reason_out);
}

fn create_placement_str(result: &DiscordRaceResultReport) -> String {
    let mut position = result.finish_position_in_class;
    position += 1;
    let emoji = match position {
        1 => " :first_place:",
        2 => " :second_place:",
        3 => " :third_place:",
        _ => ""
    };
    return format!("P{}/{}{}{}", position, result.entries_in_class, emoji, finish_reason_string(&result.reason_out));
}

fn forced_sign(n: i32) -> String {
    if n >= 0 {
        return format!("+{}", n);
    } else {
        return format!("{}", n);
    };
}

fn create_driver_str(result: &DiscordRaceResultReport) -> String {
    if result.team_name.is_empty() {
        return result.driver_name.clone();
    } else {
        return format!("{} ({})", result.driver_name, result.team_name);
    }
}

fn create_irating_str(result: &DiscordRaceResultReport) -> Option<String> {
    if result.event_type != EventType::Race {
        return None;
    }

    let irating_gain = result.newi_rating - result.oldi_rating;
    return Some(format!("{} ({})", result.newi_rating, forced_sign(irating_gain)));
}

fn create_incident_str(result: &DiscordRaceResultReport) -> Option<String> {
    if result.event_type != EventType::Race {
        return None;
    }

    let cpi_str = if result.incidents == 0 {
        "∞".to_owned()
    } else {
        let corners_complete = result.corners_per_lap * result.laps_complete;
        format!("{:.1}", (corners_complete as f32) / (result.incidents as f32))
    };
    return Some(format!("{} ({}x)", cpi_str, result.incidents));
}

fn create_track_str(result: &DiscordRaceResultReport) -> String {
    if result.config_name.is_empty() {
        return result.track_name.clone();
    } else {
        return format!("{} - {}", result.track_name, result.config_name);
    }
}

fn create_car_str(result: &DiscordRaceResultReport) -> String {
    if result.car_class_name.is_empty() {
        return result.car_name.clone();
    } else {
        return format!("{} ({})", result.car_name, result.car_class_name);
    }
}

fn create_series_str(result: &DiscordRaceResultReport) -> String {
    let session_name = if result.session_name.is_empty() {
        &result.series_name
    } else {
        &result.session_name
    };

    return format!("{} ({})", session_name, result.license_category_id.to_nice_string());
}

fn create_iracing_result_url(subsession_id: i64) -> String {
    return format!(
        "https://members.iracing.com/membersite/member/EventResult.do?&subsessionid={}",
        subsession_id
    );
}

fn create_link_line_str(team_name: &String, result: &DiscordRaceResultReport) -> String {
    let ir_history_category_str = match result.license_category_id {
        CategoryType::Road | CategoryType::FormulaCar | CategoryType::SportsCar => "road",
        CategoryType::Oval => "oval",
        CategoryType::DirtRoad => "dirt-road",
        CategoryType::DirtOval => "dirt-oval",
    };

    let encoded_team_name = urlencoding::encode(team_name.as_str());
    let encoded_driver_name = urlencoding::encode(result.driver_name.as_str());

    let session_list_url = format!(
        "https://r0mai.io/iracing-stats?team={}&type=session-list&selected={}",
        encoded_team_name,
        encoded_driver_name
    );

    let irating_history_link = format!(
        "https://r0mai.io/iracing-stats?team={}&selected={}&type=irating-history&category={}",
        encoded_team_name,
        encoded_driver_name,
        ir_history_category_str
    );

    let iracing_url = create_iracing_result_url(result.subsession_id);

    return format!("[IRacing Result]({}) | [Latest Results]({}) | [IRating History]({})",
        iracing_url,
        session_list_url,
        irating_history_link
    );
}

fn create_result_message_string(team_name: &String, result: &DiscordRaceResultReport) -> String {
    let link_line_str = create_link_line_str(team_name, result);
    let driver_str = create_driver_str(result);
    let incident_str = create_incident_str(result);
    let irating_str = create_irating_str(result);
    let placement_str = create_placement_str(result);
    let track_str = create_track_str(result);
    let car_str = create_car_str(result);
    let series_str = create_series_str(result);

    let mut lines = Vec::new();
    lines.push(format!("**Driver:**      {}", driver_str));
    lines.push(format!("**Position:**  {}", placement_str));
    lines.push(format!("**Series:**      {}", series_str));
    lines.push(format!("**Car:**           {}", car_str));
    lines.push(format!("**Track:**       {}", track_str));
    lines.push(format!("**SoF:**           {}", result.car_class_sof));
    if let Some(irating_str) = irating_str {
        lines.push(format!("**IRating:**    {}", irating_str));
    }
    if let Some(incident_str) = incident_str {
        lines.push(format!("**CPI:**           {}", incident_str));
    }

    return format!(":checkered_flag:\n{}\n\n{}",
        lines.join("\n"),
        link_line_str
    );
}

fn create_team_report_message_strings(reports: &Vec<DiscordTeamRaceResultReport>) -> Vec<String> {
    let mut messages = Vec::new();
    for (key, chunk) in &reports.into_iter().chunk_by(|report| (report.subsession_id, report.team_id, report.team_name.clone())) {
        let (subsession_id, _team_id, team_name) = key;

        let mut drivers = Vec::new();
        let iracing_url = create_iracing_result_url(subsession_id);

        for report in chunk {
            drivers.push(report.driver_name.clone());
        }

        let drivers_str = drivers.join(", ");

        messages.push(format!("**{}** ({})\n{}", team_name, drivers_str, iracing_url));
    }
    return messages;
}

pub async fn send_discord_update(subsession_ids: Vec<i64>, dry: bool) {
    let connection = create_db_connection();
    let report = query_discord_report(&connection, subsession_ids);

    // TODO iracing_client also has a request::Client. maybe we should have only one
    let client = reqwest::Client::new();

    for team in &report.individual_reports {
        for result in &team.results {
            let msg = create_result_message_string(&team.site_team_name, result);
            send_discord_message(&client, &team.hook_url, &msg, dry).await;
        }
    }

    for team in &report.team_reports {
        for message in create_team_report_message_strings(&team.results) {
            send_discord_message(&client, &team.hook_url, &message, dry).await;
        }
    }
}

pub async fn send_discord_message(client: &reqwest::Client, hook_url: &String, msg: &String, dry: bool) {
    if dry {
        println!("{}\n->\n{}", msg, hook_url);
    } else {
        let mut body = HashMap::new();
        body.insert("content", msg);

        client.post(hook_url)
            .json(&body)
            .send()
            .await.unwrap();
    }
}