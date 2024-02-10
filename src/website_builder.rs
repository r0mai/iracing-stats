use crate::db::SessionResult;

use regex::Regex;

pub fn build_website(session_results: &Vec<SessionResult>) -> String {
    let prefix = r#"
        <div class="table-2">
        <table width="100%">
        <thead>
        <tr>
        <th align="left">Esemény</th>
        <th align="left">Dátum</th>
        <th align="left">Pilóta</th>
        <th align="left">Helyzés</th>
        <th align="left">Pálya</th>
        </tr>
        </thead>
        <tbody>
    "#;

    let suffix = r#"
        </tbody>
        </table>
        </div>
    "#;

    let mut result = String::new();

    result += prefix;

    let mut current_subsession_id = -1;
    let mut current_team_id = -1;
    let mut current_series_name = String::new();
    let mut current_drivers = Vec::new();
    let mut current_track = String::new();
    let mut current_date = String::new();
    let mut current_result_str = String::new();

    for driver_result in session_results {
        if current_subsession_id != driver_result.subsession_id || current_team_id != driver_result.team_id {
            if current_subsession_id != -1 {
                let drivers_str = current_drivers.join(" ");
                result += format!(
                    r#"
                    <tr>
                    <td align="left"><a href="https://members.iracing.com/membersite/member/EventResult.do?subsessionid={}">{}</a></td>
                    <td align="left">{}</td>
                    <td align="left">{}</td>
                    <td align="left">{}</td>
                    <td align="left">{}</td>
                    </tr>
                    "#,
                current_subsession_id, current_series_name, current_date, drivers_str, current_result_str, current_track).as_str();
            }

            current_series_name = if driver_result.session_name.is_empty() {
                driver_result.series_name.clone()
            } else {
                driver_result.session_name.clone()
            };

            current_subsession_id = driver_result.subsession_id;
            current_team_id = driver_result.team_id;
            current_drivers.clear();
            current_track = driver_result.track_name.clone();
            current_date = "1.2.3".to_string(); // TODO
            current_result_str = driver_result.finish_position_in_class.to_string(); // TODO DNF/medals
        }
        current_drivers.push(driver_result.driver_name.clone());
    }
    
    result += suffix;

    // (?m) enables multiline mode
    let re = Regex::new(r"(?m)^[ \t]*").unwrap();
    result = re.replace_all(&result, "").to_string();

    return result;
}