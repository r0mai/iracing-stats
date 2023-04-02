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
    series_name TEXT NOT NULL
);

CREATE TABLE subsession(
    subsession_id INTEGER PRIMARY KEY NOT NULL,
    session_id INTEGER NOT NULL,
    start_time TEXT NOT NULL, /* 2009-11-08 16:42:29+00:00 */
    license_category_id INTEGER NOT NULL, /* 1 -> Oval, 2 -> Road, 3 -> Dirt Oval, 4 -> Dirt Road */
    event_type INTEGER NOT NULL, /* 2 -> Practice, 3 -> Qualify, 4 -> Time Trial, 5 -> Race */
    track_id INTEGER NOT NULL /* maybe should be in session? */
);

CREATE TABLE driver_result(
    cust_id INTEGER NOT NULL,
    team_id INTEGER NOT NULL,
    subsession_id INTEGER NOT NULL,
    simsession_number INTEGER NOT NULL,
    oldi_rating INTEGER NOT NULL,
    newi_rating INTEGER NOT NULL,
    old_cpi REAL NOT NULL,
    new_cpi REAL NOT NULL,
    incidents INTEGER NOT NULL,
    laps_complete INTEGER NOT NULL,
    average_lap INTEGER NOT NULL,
    car_id INTEGER NOT NULL,
    finish_position INTEGER NOT NULL, /* 0 based! */
    finish_position_in_class INTEGER NOT NULL, /* 0 based! */
    PRIMARY KEY(cust_id, team_id, subsession_id, simsession_number)
);

CREATE TABLE simsession(
    subsession_id INTEGER NOT NULL,
    simsession_number INTEGER NOT NULL, /* 0 = main event ? */
    simsession_type INTEGER NOT NULL,
    PRIMARY KEY(subsession_id, simsession_number)
);

CREATE TABLE track_config(
    track_id INTEGER PRIMARY KEY NOT NULL,
    package_id INTEGER NOT NULL, /* a.k.a track_id */
    config_name TEXT NOT NULL,
    track_config_length REAL NOT NULL, /* converted to km during db build */
    category_id INTEGER NOT NULL
);

CREATE TABLE track(
    package_id INTEGER PRIMARY KEY NOT NULL,
    track_name TEXT NOT NULL
);

CREATE TABLE car(
    car_id INTEGER PRIMARY KEY NOT NULL,
    car_name TEXT NOT NULL,
    car_name_abbreviated TEXT NOT NULL
)