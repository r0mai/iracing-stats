SELECT
    dr.cust_id,
    d.display_name,
    COUNT(DISTINCT dr.subsession_id) AS subsession_count
    /* GROUP_CONCAT(DISTINCT dr.subsession_id) AS subsession_ids */
FROM
    driver_result dr
    INNER JOIN subsession ss ON dr.subsession_id = ss.subsession_id
    INNER JOIN session s ON ss.session_id = s.session_id
    INNER JOIN driver d ON d.cust_id = dr.cust_id
WHERE
    s.season_year = 2024
    AND s.season_quarter = 3
    AND s.series_id = 237
    AND ss.event_type = 5
    AND (
        dr.livery_sponsor1 IN (172, 173)
        OR dr.livery_sponsor2 IN (172, 173)
    )
GROUP BY
    dr.cust_id
HAVING
    subsession_count >= 4
ORDER BY
    subsession_count DESC;