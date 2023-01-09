.headers on
SELECT
    subsession.subsession_id,
    session.series_name, 
    start_time,
    track.track_name,
    driver_result.oldi_rating
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
    driver.display_name = "Andras Kucsma" AND
    subsession.event_type == 5 AND
    simsession.simsession_number == 0 AND
    driver_result.finish_position_in_class == 0
ORDER BY
    start_time