SELECT 
    "site_team"."site_team_name", 
    "site_team"."team_report_discord_hook_url", 
    "driver"."display_name", 
    "subsession"."subsession_id", 
    "session"."series_name", 
    "session"."session_name", 
    "car"."car_name", 
    "track_config"."track_name", 
    "track_config"."config_name", 
    "track_config"."corners_per_lap", 
    "driver_result"."finish_position_in_class", 
    "driver_result"."incidents", 
    "driver_result"."oldi_rating", 
    "driver_result"."newi_rating", 
    "driver_result"."laps_complete", 
    "subsession"."event_type", 
    "reason_out"."reason_out", 
    "car_class_result"."entries_in_class", 
    "driver_result"."team_name", 
    (CASE 
        WHEN ("car_class"."car_class_id" = 0 
            OR "car_class"."car_class_id" = -1 
            OR "car_class"."car_class_size" <= 1) 
        THEN ""
        ELSE "car_class"."car_class_name" 
    END) AS "car_class_display_name", 
    "car_class_result"."class_sof", 
    "subsession"."license_category_id", 
    "driver_result"."team_id"
FROM 
    "driver_result"
INNER JOIN 
    "subsession" 
    ON "driver_result"."subsession_id" = "subsession"."subsession_id"
INNER JOIN 
    "simsession" 
    ON "driver_result"."subsession_id" = "simsession"."subsession_id" 
    AND "driver_result"."simsession_number" = "simsession"."simsession_number"
INNER JOIN 
    "driver" 
    ON "driver_result"."cust_id" = "driver"."cust_id"
INNER JOIN 
    "car" 
    ON "driver_result"."car_id" = "car"."car_id"
INNER JOIN 
    "reason_out" 
    ON "driver_result"."reason_out_id" = "reason_out"."reason_out_id"
INNER JOIN 
    "session" 
    ON "session"."session_id" = "subsession"."session_id"
INNER JOIN 
    "track_config" 
    ON "track_config"."track_id" = "subsession"."track_id"
INNER JOIN 
    "site_team_team" 
    ON "driver_result"."team_id" = "site_team_team"."team_id"
INNER JOIN 
    "site_team" 
    ON "site_team"."site_team_id" = "site_team_team"."site_team_id"
INNER JOIN 
    "car_class_result" 
    ON "driver_result"."subsession_id" = "car_class_result"."subsession_id" 
    AND "driver_result"."simsession_number" = "car_class_result"."simsession_number" 
    AND "driver_result"."car_class_id" = "car_class_result"."car_class_id"
INNER JOIN 
    "car_class" 
    ON "driver_result"."car_class_id" = "car_class"."car_class_id"
WHERE 
    "driver_result"."subsession_id" IN (70671402)
    AND "simsession"."simsession_type" = 6
    AND "site_team"."team_report_discord_hook_url" IS NOT NULL
ORDER BY 
    "subsession"."subsession_id" ASC, 
    "driver_result"."team_id" ASC;