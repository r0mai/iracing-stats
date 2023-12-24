CREATE INDEX driver_driver_name_index ON driver(display_name);
CREATE INDEX driver_result_team_id_index ON driver_result(team_id);
CREATE INDEX driver_result_subsession_id_index ON driver_result(subsession_id);
CREATE INDEX site_team_site_team_name_index ON site_team(site_team_name);
CREATE INDEX site_team_member_cust_id_index ON site_team_member(cust_id);
CREATE INDEX subsession_start_time_index ON subsession(start_time);