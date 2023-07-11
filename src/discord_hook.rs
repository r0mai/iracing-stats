use std::collections::HashMap;

use crate::{db::{query_discord_report, create_db_connection, DiscordSiteTeamReport, DiscordResultReport}, event_type::EventType};

fn placement_string(mut position: i32) -> String {
    position += 1;
    let emoji = match position {
        1 => " :first_place:",
        2 => " :second_place:",
        3 => " :third_place:",
        _ => ""
    };
    return format!("**P{}**{}", position, emoji);
}

fn create_result_message_string(team_name: &String, result: &DiscordResultReport) -> String {
    let r0mai_io_url = format!(
        "http://r0mai.io/iracing-stats?team={}&type=session-list&selected={}",
        urlencoding::encode(team_name.as_str()),
        urlencoding::encode(result.driver_name.as_str())
    );

    let iracing_url = format!(
        "https://members.iracing.com/membersite/member/EventResult.do?&subsessionid={}",
        result.subsession_id
    );

    let race_details_str = if result.event_type == EventType::Race {
        let cpi_str = if result.incidents == 0 {
            "∞".to_owned()
        } else {
            let corners_complete = result.corners_per_lap * result.laps_complete;
            format!("{:.1}", (corners_complete as f32) / (result.incidents as f32))
        };
        let irating_gain = result.newi_rating - result.oldi_rating;
        let irating_gain_str = if irating_gain > 0 {
            format!("+{}", irating_gain)
        } else {
            format!("{}", irating_gain)
        };
        format!("IRating: {} ({}), CPI: {} ({}x)\n", result.newi_rating, irating_gain_str, cpi_str, result.incidents)
    } else {
        "".to_owned()
    };

    return format!(
        "**{}** finished {} in **{}** [{}] :race_car: {} :motorway: {}\n{}<{}>\n<{}>",
        result.driver_name,
        placement_string(result.finish_position_in_class),
        result.series_name,
        result.event_type.to_nice_string(),
        result.car_name,
        result.track_name,
        race_details_str,
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