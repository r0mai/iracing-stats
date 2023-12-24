SELECT display_name, irating, subsession_id
FROM (
    SELECT
        "driver"."display_name",
        "driver_result"."oldi_rating" as irating,
        subsession.subsession_id as subsession_id,
        ROW_NUMBER() OVER (PARTITION BY driver.display_name ORDER BY subsession.start_time ASC) AS row_num
    FROM "driver_result"
    INNER JOIN "subsession" ON "driver_result"."subsession_id" = "subsession"."subsession_id"
    INNER JOIN "simsession" ON "driver_result"."subsession_id" = "simsession"."subsession_id" AND "driver_result"."simsession_number" = "simsession"."simsession_number"
    INNER JOIN "driver" ON "driver_result"."cust_id" = "driver"."cust_id"
    INNER JOIN "session" ON "session"."session_id" = "subsession"."session_id"
    INNER JOIN "track_config" ON "track_config"."track_id" = "subsession"."track_id"
    INNER JOIN "site_team_member" ON "driver"."cust_id" = "site_team_member"."cust_id"
    INNER JOIN "site_team" ON "site_team"."site_team_id" = "site_team_member"."site_team_id"
    WHERE
        "site_team"."site_team_name" = "rsmr" AND
        "driver_result"."newi_rating" <> -1 AND
        "driver_result"."oldi_rating" <> -1 AND
        "subsession"."start_time" >= "2023-01-01 00:00:00+00:00" AND
        "subsession"."start_time" < "2024-01-01 00:00:00+00:00" AND
        "track_config"."category_id" = 2 AND
        "subsession"."event_type" = 5 AND
        "subsession"."official_session"
) WHERE
    row_num = 1
;