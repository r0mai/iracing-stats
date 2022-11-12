import sqlite3
import datetime
import json
import os

from common import *

def parse_date(date):
    dt = datetime.datetime.strptime(date, "%Y-%m-%dT%H:%M:%SZ")
    return int((dt - datetime.datetime(1970, 1, 1)) / datetime.timedelta(seconds=1))


def build_db_schema(cur):
    cur.execute(
        '''CREATE TABLE driver(
            cust_id INTEGER UNIQUE,
            display_name TEXT
        )'''
    )
    cur.execute('''CREATE UNIQUE INDEX driver_index ON driver(display_name)''')
            
    cur.execute(
        '''CREATE TABLE session(
            session_id INTEGER PRIMARY KEY,
            series_name TEXT
        )'''
    )

    cur.execute(
        '''CREATE TABLE subsession(
            subsession_id INTEGER PRIMARY KEY,
            session_id INTEGER,
            start_time INTEGER,
            license_category_id INTEGER,
            track_id INTEGER /* maybe should be in session? */
        )'''
    )
    cur.execute(
        '''CREATE TABLE driver_result(
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
        )'''
    )
    cur.execute(
        '''CREATE TABLE simsession(
            subsession_id INTEGER,
            simsession_number INTEGER,
            simsession_type INTEGER,
            PRIMARY KEY(subsession_id, simsession_number)
        )'''
    )
    cur.execute(
        '''CREATE TABLE track_config(
            track_id INTEGER PRIMARY KEY,
            package_id INTEGER, /* a.k.a track_id */
            config_name TEXT,
            track_config_length REAL
        )'''
    )
    cur.execute(
        '''CREATE TABLE track(
            package_id INTEGER PRIMARY KEY,
            track_name TEXT
        )'''
    )
    cur.execute(
        '''CREATE TABLE car(
            car_id INTEGER PRIMARY_KEY,
            car_name TEXT,
            car_name_abbreviated TEXT
        )'''
    )


def add_driver_to_db(cur, driver_result):
    cur.execute(
        '''INSERT OR IGNORE INTO driver VALUES(
            ?, /* cust_id */
            ?  /* display_name */
        )''', (
            driver_result['cust_id'],
            driver_result['display_name'],
        )
    )

def add_session_to_db(cur, subsession):
    cur.execute(
        '''INSERT OR IGNORE INTO session VALUES(
            ?,  /* session_id */
            ?   /* series_name */
        )''', (subsession['session_id'], subsession['series_name'])
    )

def add_driver_result_to_db(cur, subsession_id, simsession_number, team_id, driver_result):
    cust_id = driver_result['cust_id']

    add_driver_to_db(cur, driver_result)

    cur.execute(
        '''INSERT INTO driver_result VALUES(
            ?, /* cust_id */
            ?, /* team_id */
            ?, /* subsession_id */
            ?, /* simsession_number */
            ?, /* newi_rating */
            ?, /* incidents */
            ?, /* laps_complete */
            ?, /* average_lap */
            ?  /* car_id */
        )''', (
            cust_id,
            team_id,
            subsession_id,
            simsession_number,
            driver_result['newi_rating'],
            driver_result['incidents'],
            driver_result['laps_complete'],
            driver_result['average_lap'],
            driver_result['car_id']
        )
    )

def add_simsession_to_db(cur, subsession_id, simsession):
    simsession_number = simsession['simsession_number']

    cur.execute(
        '''INSERT INTO simsession VALUES(
            ?, /* subsession_id */
            ?, /* simsession_number */
            ?  /* simsession_type */
        )''', (
            subsession_id,
            simsession_number,
            simsession['simsession_type']
        )
    )

    for participant in simsession['results']:
        if 'cust_id' in participant:
            add_driver_result_to_db(cur, subsession_id, simsession_number, 0, participant);
        else: # team
            team_id = participant['team_id']
            for driver in participant['driver_results']:
                add_driver_result_to_db(cur, subsession_id, simsession_number, team_id, driver);


def add_subsession_to_db(cur, subsession):
    subsession_id = subsession['subsession_id']

    cur.execute(
        '''INSERT INTO subsession VALUES(
            ?, /* subsession_id */
            ?, /* session_id */
            ?, /* start_time */
            ?, /* license_category_id */
            ?  /* track_id */
        )''', (
            subsession_id,
            subsession['session_id'],
            parse_date(subsession['start_time']),
            subsession['license_category_id'],
            subsession['track']['track_id']
        )
    )

    add_session_to_db(cur, subsession)

    for simsession in subsession['session_results']:
        add_simsession_to_db(cur, subsession_id, simsession)

def add_track_to_db(cur, track):
    cur.execute(
        '''INSERT OR IGNORE INTO track VALUES(
            ?, /* package_id */
            ?  /* track_name */
        )''', (
            track['package_id'],
            track['track_name']
        )
    )

    cur.execute(
        '''INSERT INTO track_config VALUES(
            ?, /* track_id */
            ?, /* package_id */
            ?, /* config_name */
            ?  /* track_config_length */
        )''', (
            track['track_id'],
            track['package_id'],
            track.get('config_name', ''),
            track['track_config_length']
        )
    )

def add_car_to_db(cur, car):
    cur.execute(
        '''INSERT INTO car VALUES(
            ?, /* car_id */
            ?, /* car_name */
            ?  /* car_name_abbreviated */
        )''', (
            car['car_id'],
            car['car_name'],
            car['car_name_abbreviated']
        )
    )

def query_irating_history(driver_name):
    con = sqlite3.connect(SQLITE_DB_FILE)
    cur = con.cursor()

    rows = cur.execute(
        '''SELECT subsession.start_time, driver_result.newi_rating, session.series_name FROM
            driver_result
            JOIN simsession ON
                driver_result.subsession_id = simsession.subsession_id AND
                driver_result.simsession_number = simsession.simsession_number
            JOIN subsession ON
                simsession.subsession_id = subsession.subsession_id
            JOIN session ON
                subsession.session_id = session.session_id
            WHERE
                driver_result.cust_id = (SELECT cust_id FROM driver WHERE display_name = ?) AND
                driver_result.newi_rating != -1 AND
                subsession.license_category_id = 2
            ORDER BY subsession.start_time ASC;
        ''', (driver_name,)
    )

    result = []

    for row in rows:
        result.append(dict(
            start_time = row[0],
            irating = row[1],
            series_name = row[2]
        ))

    return result

def query_track_car_usage_matrix(driver_name):
    con = sqlite3.connect(SQLITE_DB_FILE)
    cur = con.cursor()

    rows = cur.execute(
        ''' SELECT
                car.car_name_abbreviated,
                track.track_name,
                driver_result.laps_complete * driver_result.average_lap
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
                driver.display_name = ?
            GROUP BY
                driver_result.car_id, track.package_id
        ''', (driver_name,)
    )

    result = []

    for row in rows:
        result.append(dict(
            car_name = row[0],
            track_name = row[1],
            total_time = row[2]
        ))

    return result


def rebuild_sessions(con, cur):
    i = 0
    files = os.listdir(SESSIONS_DIR)
    for session_file in files:
        if i % 1000 == 0:
            print('{0}/{1}'.format(i, len(files)))
            con.commit()
        i += 1
        with open(os.path.join(SESSIONS_DIR, session_file), 'r') as file:
            data = json.load(file)
            add_subsession_to_db(cur, data)

def rebuild_tracks(cur):
    with open(TRACK_DATA_FILE, 'r') as file:
        tracks = json.load(file)

    for track in tracks:
        add_track_to_db(cur, track)

def rebuild_cars(cur):
    with open(CAR_DATA_FILE, 'r') as file:
        cars = json.load(file)

    for car in cars:
        add_car_to_db(cur, car)


def rebuild_db():
    os.remove(SQLITE_DB_FILE)

    con = sqlite3.connect(SQLITE_DB_FILE)
    cur = con.cursor()
    build_db_schema(cur)

    rebuild_tracks(cur)
    rebuild_cars(cur)
    rebuild_sessions(con, cur)

    con.commit()
