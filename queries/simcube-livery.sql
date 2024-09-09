SELECT
    driver_result.cust_id,
    driver.display_name,
    COUNT(DISTINCT driver_result.subsession_id) AS subsession_count
FROM
    driver_result
    INNER JOIN subsession ON driver_result.subsession_id = subsession.subsession_id
    INNER JOIN session ON subsession.session_id = session.session_id
    INNER JOIN driver ON driver.cust_id = driver_result.cust_id
WHERE
    session.season_year = 2024
    AND session.season_quarter = 3
    AND session.series_id = 237
    AND subsession.event_type = 5
    AND (
        driver_result.livery_sponsor1 IN (172, 173)
        OR driver_result.livery_sponsor2 IN (172, 173)
    )
GROUP BY
    driver_result.cust_id
HAVING
    subsession_count >= 4
ORDER BY
    subsession_count DESC;