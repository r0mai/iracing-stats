.mode csv
.headers on
SELECT 
    car_name,
    COUNT(DISTINCT display_name)
FROM (
    SELECT
        car_name,
        driver.display_name,
        /* COUNT(DISTINCT subsession.subsession_id), */
        /* round(SUM(track_config_length * driver_result.laps_complete)) */
        SUM(driver_result.average_lap * driver_result.laps_complete) / 10000 / 60 as "minutes"

    FROM
        driver_result
    JOIN simsession ON
        driver_result.subsession_id = simsession.subsession_id AND
        driver_result.simsession_number = simsession.simsession_number
    JOIN subsession ON
        simsession.subsession_id = subsession.subsession_id
    JOIN session ON
        subsession.session_id = session.session_id
    JOIN track_config ON
        subsession.track_id = track_config.track_id
    JOIN car ON
        driver_result.car_id = car.car_id
    JOIN driver ON
        driver.cust_id = driver_result.cust_id
    JOIN site_team_member ON
        site_team_member.cust_id = driver.cust_id
    JOIN site_team ON
        site_team.site_team_id = site_team_member.site_team_id
    WHERE
        site_team_name = "rsmr" AND
        simsession.simsession_type = 6 AND
        /* driver_result.team_id != 0 AND */
        subsession.start_time LIKE '2023%'
    GROUP BY
        car.car_id,
        driver.cust_id
    HAVING
        SUM(driver_result.average_lap * driver_result.laps_complete) / 10000 / 60 >= 90
    ORDER BY
        car_name ASC,
        SUM(driver_result.average_lap * driver_result.laps_complete) DESC
 )
 GROUP BY
    car_name
ORDER BY
    COUNT(DISTINCT display_name) DESC

;
