explain query plan
select
    site_team.discord_hook_url,
    driver.display_name,
    subsession.subsession_id,
    session.series_name,
    car.car_name,
    track.track_name,
    driver_result.finish_position_in_class+1
from driver_result
        inner join subsession on driver_result.subsession_id = subsession.subsession_id
        inner join simsession on driver_result.subsession_id = simsession.subsession_id and driver_result.simsession_number = simsession.simsession_number
        inner join driver on driver_result.cust_id = driver.cust_id
        inner join session on subsession.session_id = session.session_id
        inner join car on driver_result.car_id = car.car_id
        inner join track_config on subsession.track_id = track_config.track_id
        inner join track on track_config.package_id = track.package_id
        inner join site_team_member on driver_result.cust_id = site_team_member.cust_id
        inner join site_team on site_team_member.site_team_id = site_team.site_team_id
where 1
    and subsession.subsession_id in (61268227, 61145537)
    and simsession.simsession_number = 0 /* is main event */
    and subsession.event_type = 5 /* EventType is race */
    and site_team.discord_hook_url NOT NULL
    ;