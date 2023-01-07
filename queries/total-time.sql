SELECT
    SUM(driver_result.laps_complete * track_config.track_config_length),
    SUM(driver_result.laps_complete * driver_result.average_lap) / 10000 / 3600,
    SUM(driver_result.laps_complete)
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
JOIN track ON
    track_config.package_id = track.package_id
JOIN car ON
    driver_result.car_id = car.car_id
JOIN driver ON
    driver.cust_id = driver_result.cust_id
WHERE
    driver.display_name = "Andras Kucsma"
