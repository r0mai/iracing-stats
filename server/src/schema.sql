CREATE TABLE driver(
    cust_id INTEGER PRIMARY KEY NOT NULL,
    display_name TEXT NOT NULL
);

CREATE TABLE season(
    season_id INTEGER PRIMARY KEY NOT NULL,
    series_id INTEGER NOT NULL,
    season_name TEXT NOT NULL,
    series_name TEXT NOT NULL,
    official INTEGER NOT NULL, /* boolean */
    season_year INTEGER NOT NULL,
    season_quarter INTEGER NOT NULL, /* 1,2,3,4 */
    license_group_id INTEGER NOT NULL, /* 1 -> Oval, 2 -> Road, 3 -> Dirt Oval, 4 -> Dirt Road */
    fixed_setup INTEGER NOT NULL, /* boolean */
    driver_changes INTEGER NOT NULL /* boolean */
);

CREATE TABLE session(
    session_id INTEGER PRIMARY KEY NOT NULL,
    series_name TEXT NOT NULL,
    session_name TEXT, /* may be null; only exists for Hosted races */
    season_year INTEGER NOT NULL,
    season_quarter INTEGER NOT NULL, /* 1 based */
    series_id INTEGER NOT NULL 
);

CREATE TABLE subsession(
    subsession_id INTEGER PRIMARY KEY NOT NULL,
    session_id INTEGER NOT NULL,
    start_time TEXT NOT NULL, /* 2009-11-08 16:42:29+00:00 */
    license_category_id INTEGER NOT NULL, /* 1 -> Oval, 2 -> Road, 3 -> Dirt Oval, 4 -> Dirt Road */
    event_type INTEGER NOT NULL, /* 2 -> Practice, 3 -> Qualify, 4 -> Time Trial, 5 -> Race */
    track_id INTEGER NOT NULL, /* maybe should be in session? */
    official_session BOOLEAN NOT NULL /* hosted vs. official; maybe should be in session? */
);

CREATE TABLE driver_result(
    cust_id INTEGER NOT NULL,
    team_id INTEGER NOT NULL,
    team_name TEXT NOT NULL, /* TODO could deduplicate with https://stackoverflow.com/questions/65343126/add-data-to-many-to-many-relation-with-one-sql-command/65357904 */
    subsession_id INTEGER NOT NULL,
    simsession_number INTEGER NOT NULL, /* 0 -> Main event, negative values are pre-events */
    oldi_rating INTEGER NOT NULL,
    newi_rating INTEGER NOT NULL,
    old_cpi REAL NOT NULL,
    new_cpi REAL NOT NULL,
    incidents INTEGER NOT NULL,
    laps_complete INTEGER NOT NULL,
    average_lap INTEGER NOT NULL,
    car_id INTEGER NOT NULL,
    car_class_id INTEGER NOT NULL,
    finish_position INTEGER NOT NULL, /* 0 based! */
    finish_position_in_class INTEGER NOT NULL, /* 0 based! */
    reason_out_id INTEGER NOT NULL,
    champ_points INTEGER NOT NULL,
    division INTEGER NOT NULL, /* 0 -> Division 1, etc */
    livery_sponsor1 INTEGER NOT NULL,
    livery_sponsor2 INTEGER NOT NULL,
    starting_position INTEGER NOT NULL,
    starting_position_in_class INTEGER NOT NULL,
    PRIMARY KEY(cust_id, team_id, subsession_id, simsession_number)
);

CREATE TABLE car_class(
    car_class_id INTEGER PRIMARY KEY NOT NULL,
    car_class_name TEXT NOT NULL,
    car_class_short_name TEXT NOT NULL,
    car_class_size INTEGER NOT NULL /* number of vehicles in class */
);

CREATE TABLE car_class_member(
    car_class_id INTEGER NOT NULL,
    car_id INTEGER NOT NULL
);

CREATE TABLE car_class_result(
    car_class_id INTEGER NOT NULL,
    subsession_id INTEGER NOT NULL,
    simsession_number INTEGER NOT NULL,
    entries_in_class INTEGER NOT NULL,
    class_sof INTEGER NOT NULL,
    PRIMARY KEY(subsession_id, simsession_number, car_class_id)
);

CREATE TABLE simsession(
    subsession_id INTEGER NOT NULL,
    simsession_number INTEGER NOT NULL, /* 0 = main event ? */
    simsession_type INTEGER NOT NULL, /* 3 -> Open Practice, 4 -> Lone Qualifying, 6 -> Race */
    entries INTEGER NOT NULL,
    sof INTEGER NOT NULL,
    PRIMARY KEY(subsession_id, simsession_number)
);

CREATE TABLE reason_out(
    reason_out_id INTEGER PRIMARY KEY NOT NULL,
    reason_out TEXT NOT NULL
);

CREATE TABLE track_config(
    track_id INTEGER PRIMARY KEY NOT NULL,
    package_id INTEGER NOT NULL, /* a.k.a track_id */
    track_name TEXT NOT NULL,
    config_name TEXT NOT NULL,
    track_config_length REAL NOT NULL, /* converted to km during db build */
    corners_per_lap INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    grid_stalls INTEGER NOT NULL,
    pit_road_speed_limit INTEGER NOT NULL, /* converted to km/h during db build */
    number_pitstalls INTEGER NOT NULL
);

CREATE TABLE car(
    car_id INTEGER PRIMARY KEY NOT NULL,
    car_name TEXT NOT NULL,
    car_name_abbreviated TEXT NOT NULL
);

CREATE TABLE site_team(
    site_team_id INTEGER PRIMARY KEY NOT NULL,
    site_team_name TEXT NOT NULL,
    discord_hook_url TEXT, /* may be null */
    team_report_discord_hook_url TEXT /* may be null */
);

CREATE TABLE site_team_member(
    site_team_id INTEGER NOT NULL,
    cust_id INTEGER NOT NULL
);

CREATE TABLE site_team_team(
    site_team_id INTEGER NOT NULL,
    team_id INTEGER NOT NULL
)