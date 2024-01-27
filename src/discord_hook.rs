use std::collections::HashMap;

use crate::{db::{query_discord_report, create_db_connection, DiscordResultReport}, event_type::EventType};

fn finish_reason_string(reason_out: &String) -> String {
    if reason_out == "Running" {
        return "".to_string();
    }

    if reason_out == "" {
        return " [Unknown out reason]".to_string();
    }

    return format!(" [{}]", reason_out);
}

fn create_placement_str(result: &DiscordResultReport) -> String {
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

fn create_driver_str(result: &DiscordResultReport) -> String {
    if result.team_name.is_empty() {
        return result.driver_name.clone();
    } else {
        return format!("{} ({})", result.driver_name, result.team_name);
    }
}

fn create_irating_str(result: &DiscordResultReport) -> Option<String> {
    if result.event_type != EventType::Race {
        return None;
    }

    let irating_gain = result.newi_rating - result.oldi_rating;
    return Some(format!("{} ({})", result.newi_rating, forced_sign(irating_gain)));
}

fn create_incident_str(result: &DiscordResultReport) -> Option<String> {
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

fn create_track_str(result: &DiscordResultReport) -> String {
    if result.config_name.is_empty() {
        return result.track_name.clone();
    } else {
        return format!("{} - {}", result.track_name, result.config_name);
    }
}

fn create_result_message_string(team_name: &String, result: &DiscordResultReport) -> String {
    let r0mai_io_url = format!(
        "https://r0mai.io/iracing-stats?team={}&type=session-list&selected={}",
        urlencoding::encode(team_name.as_str()),
        urlencoding::encode(result.driver_name.as_str())
    );

    let iracing_url = format!(
        "https://members.iracing.com/membersite/member/EventResult.do?&subsessionid={}",
        result.subsession_id
    );

    let driver_str = create_driver_str(result);
    let incident_str = create_incident_str(result);
    let irating_str = create_irating_str(result);
    let placement_str = create_placement_str(result);
    let track_str = create_track_str(result);

    let race_name_str = if result.session_name.is_empty() {
        &result.series_name
    } else {
        &result.session_name
    };

    let mut lines = Vec::new();
    lines.push(format!("**Driver:**      {}", driver_str));
    lines.push(format!("**Position:**  {}", placement_str));
    lines.push(format!("**Series:**      {}", race_name_str));
    lines.push(format!("**Car:**           {}", result.car_name));
    lines.push(format!("**Track:**       {}", track_str));
    if let Some(irating_str) = irating_str {
        lines.push(format!("**IRating:**    {}", irating_str));
    }
    if let Some(incident_str) = incident_str {
        lines.push(format!("**CPI:**           {}", incident_str));
    }

    return format!(":checkered_flag:\n{}\n\n{}\n{}",
        lines.join("\n"),
        r0mai_io_url,
        iracing_url
    );
}

pub async fn send_discord_update(subsession_ids: Vec<i64>, dry: bool) {
    let connection = create_db_connection();
    let teams = query_discord_report(&connection, subsession_ids);

    // TODO iracing_client also has a request::Client. maybe we should have only one
    let client = reqwest::Client::new();

    for team in &teams.teams {
        for result in &team.results {
            let msg = create_result_message_string(&team.site_team_name, result);
            send_discord_message(&client, &team.hook_url, &msg, dry).await;
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