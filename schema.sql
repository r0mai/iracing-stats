CREATE TABLE driver(
    cust_id INTEGER UNIQUE,
    display_name TEXT
);

CREATE TABLE session(
    session_id INTEGER PRIMARY KEY,
    series_name TEXT
);

CREATE TABLE subsession(
    subsession_id INTEGER PRIMARY KEY,
    session_id INTEGER,
    start_time INTEGER,
    license_category_id INTEGER,
    track_id INTEGER /* maybe should be in session? */
);

CREATE TABLE driver_result(
    cust_id INTEGER,
    team_id INTEGER,
    subsession_id INTEGER,
    simsession_number INTEGER,
    newi_rating INTEGER,
    incidents INTEGER,
    laps_complete INTEGER,
    average_lap INTEGER,
    car_id INTEGER,
    PRIMARY KEY(cust_id, team_id, subsession_id, simsession_number)
);

CREATE TABLE simsession(
    subsession_id INTEGER,
    simsession_number INTEGER,
    simsession_type INTEGER,
    PRIMARY KEY(subsession_id, simsession_number)
);

CREATE TABLE track_config(
    track_id INTEGER PRIMARY KEY,
    package_id INTEGER, /* a.k.a track_id */
    config_name TEXT,
    track_config_length REAL
);

CREATE TABLE track(
    package_id INTEGER PRIMARY KEY,
    track_name TEXT
);

CREATE TABLE car(
    car_id INTEGER PRIMARY_KEY,
    car_name TEXT,
    car_name_abbreviated TEXT
)