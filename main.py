import os
import sys
import asyncio
import aiohttp
import hashlib
import base64
import json
import sqlite3
import argparse
import matplotlib.pyplot as plt
import numpy as np
import datetime

BASEURL = 'https://members-ng.iracing.com'

SESSIONS_DIR = 'sessions'

SQLITE_DB_FILE = 'stats.db'

async def get_json(s, suffix, params):
    while True:
        async with s.get(BASEURL + suffix, params=params) as res:
            if res.status == 429: # we get rate limited
                print('Rate limited, sleep 5 seconds')
                await asyncio.sleep(5)
            elif res.status == 200:
                return await res.json()
            else:
                print('Request {0} {1} failed'.format(suffix, params), res)
                raise 'error'

async def get_and_read(s, suffix, params):
    res = await get_json(s, suffix, params)
    async with s.get(res['link']) as res:
        return await res.json()

async def get_and_read_chunked(s, suffix, params):
    res = await get_json(s, suffix, params)
    chunk_info = res['data']['chunk_info']
    base_url = chunk_info['base_download_url']
    result_array = []
    for file in chunk_info['chunk_file_names']:
        url = base_url + file
        async with s.get(url) as res:
            result_array += await res.json()

    return result_array

async def get_cust_id(s, search_term):
    res = await get_and_read(s, '/data/lookup/drivers', {'search_term': search_term})
    if len(res) == 0:
        raise 'Not found'
    if len(res) > 1:
        print('Multple drivers found {0}'.format(len(res)))
    return res[0]['cust_id']

async def get_member_since(s, cust_id):
    res = await get_and_read(s, '/data/member/get', {'cust_ids': cust_id})
    return res['members'][0]['member_since']

async def search_series(s, cust_id, year, quarter):
    return await get_and_read_chunked(s, '/data/results/search_series', {
        'cust_id': cust_id,
        'season_year': year,
        'season_quarter': quarter
    })

async def get_session_results(s, subsession_id):
    cached_path = os.path.join(SESSIONS_DIR, '{0}.session'.format(subsession_id))
    if os.path.exists(cached_path):
        with open(cached_path, 'r') as file:
            return json.load(file)

    print('Syncing session {0}'.format(subsession_id))
    result = await get_and_read(s, '/data/results/get/', {'subsession_id': subsession_id})

    with open(cached_path, 'w') as file:
        json.dump(result, file)

    return result

def get_time_spent_in_session(session_results, cust_id):
    time_spent = 0
    for session_result in session_results['session_results']:
        for participant in session_result['results']:
            if 'cust_id' in participant:
                if participant['cust_id'] == cust_id:
                    time_spent += participant['average_lap'] * participant['laps_complete']
            else: # team
                for driver in participant['driver_results']:
                    if driver['cust_id'] == cust_id:
                        time_spent += driver['average_lap'] * driver['laps_complete']

    return time_spent

def get_laps_completed_in_session(session_results, cust_id):
    laps_completed = 0
    for session_result in session_results['session_results']:
        for participant in session_result['results']:
            if 'cust_id' in participant:
                if participant['cust_id'] == cust_id:
                    laps_completed += participant['laps_complete']
            else: # team
                for driver in participant['driver_results']:
                    if driver['cust_id'] == cust_id:
                        laps_completed += driver['laps_complete']


    return laps_completed 

def get_car_used_in_session(session_results, cust_id):
    for session_result in session_results['session_results']:
        # participant may be a team or a player
        for participant in session_result['results']:
            if 'cust_id' in participant:
                if participant['cust_id'] == cust_id:
                    return participant['car_name']
            else: # team
                for driver in participant['driver_results']:
                    if driver['cust_id'] == cust_id:
                        return participant['car_name']


    return 'Unknown car'


def get_series_name(session_results):
    return session_results['season_name']

def get_start_time(session_results):
    return session_results['start_time']

def get_track_id(session_results):
    return session_results['track']['track_id']

async def get_track_infos(s):
    data = await get_and_read(s, '/data/track/get', {})

    result = {}

    for track_data in data:
        result[track_data['track_id']] = track_data

    return result

def get_track_length(track_infos, track_id):
    track = track_infos[track_id]
    return track['track_config_length']

def get_full_track_name(track_infos, track_id):
    track = track_infos[track_id]
    return '{0} -- {1}'.format(track['track_name'], track['config_name'])

def get_track_name(track_infos, track_id):
    track = track_infos[track_id]
    return track['track_name']

def get_track_price(track_infos, track_id):
    return track_infos[track_id]['price']

def encode_pw(username, password):
    initialHash = hashlib.sha256((password + username.lower()).encode('utf-8')).digest()
    hashInBase64 = base64.b64encode(initialHash).decode('utf-8')
    return hashInBase64

def to_hours(interval):
    return interval / 10000 / 60 / 60

async def collect_cumulative_data(s, series, track_infos, cust_id):
    time_spent = 0
    length_driven = 0

    for ser in series:
        session_result = await get_session_results(s, ser['subsession_id'])

        track_id = get_track_id(session_result)
        track_length = get_track_length(track_infos, track_id)
        time = get_time_spent_in_session(session_result, cust_id)
        kms = track_length * get_laps_completed_in_session(session_result, cust_id)

        # print('Processing {0} {1} ({2}) -- {3}s | {4}km'.format(
        #     get_start_time(session_result),
        #     get_series_name(session_result),
        #     ser['subsession_id'],
        #     time / 10000,
        #     kms)
        # )

        time_spent += time
        length_driven += kms

    hours = to_hours(time_spent)
    print('Time spent: {0:.1f} hours'.format(hours))
    print('Length driven: {0:.1f}km'.format(length_driven))
    print('Average speed: {0:.1f}km/h'.format(length_driven / hours))



class TrackCarData:
    def __init__(self):
        self._track_set = set()
        self._car_set = set()

        # data[track_name][car_name]
        self.data = dict()
        self.car_sums = dict()
        self.track_sums = dict()
        self.sum_total = 0

    def add_data(self, track_name, car_name, data):
        self._ensure_track(track_name)
        self._ensure_car(car_name)
        if self.data[track_name][car_name] is not None:
            self.data[track_name][car_name] += data
        else:
            self.data[track_name][car_name] = data

        self.track_sums[track_name] += data
        self.car_sums[car_name] += data
        self.sum_total += data

    def _ensure_track(self, track_name):
        if track_name not in self._track_set:
            self._track_set.add(track_name)
            self.data[track_name] = dict.fromkeys(self._car_set, None)
            self.track_sums[track_name] = 0

    def _ensure_car(self, car_name):
        if car_name not in self._car_set:
            self._car_set.add(car_name)
            for track_name, cars in self.data.items():
                cars[car_name] = None 
            self.car_sums[car_name] = 0

    def to_table(self):
        car_indices = dict()
        track_indices = dict()

        sorted_cars = sorted(self.car_sums.items(), key = lambda p: p[1], reverse=True)
        sorted_tracks = sorted(self.track_sums.items(), key = lambda p: p[1], reverse=True)

        for track_name, _ in sorted_tracks:
            track_indices[track_name] = len(track_indices)

        for car_name, _ in sorted_cars:
            car_indices[car_name] = len(car_indices)

        table = []
        for i in range(0, len(track_indices)):
            table.append([None] * len(car_indices))

        for track, cars in self.data.items():
            for car, time in cars.items():
                if time is not None:
                    table[track_indices[track]][car_indices[car]] = to_hours(time)

        # add Sums
        table.append([None] * len(car_indices))
        for row in table:
            row.append(None)

        for car, s in sorted_cars:
            table[-1][car_indices[car]] = to_hours(s)

        for track, s in sorted_tracks:
            table[track_indices[track]][-1] = to_hours(s)

        table[-1][-1] = to_hours(self.sum_total)

        car_labels = [''] * len(car_indices)
        for car_name, idx in car_indices.items():
            car_labels[idx] = car_name

        car_labels.append('SUM')

        track_labels = [''] * len(track_indices)
        for track_name, idx in track_indices.items():
            track_labels[idx] = track_name

        track_labels.append('SUM')

        return track_labels, car_labels, table

def get_largest_element_of_table(table):
    largest = 0
    for a in table[:-1]:
        for b in a[:-1]:
            if b is not None and b > largest:
                largest = b

    return largest

def mix_colors(a, b, t):
    res = [0] * len(a)
    i = 0
    for i in range(len(a)):
        res[i] = t * b[i] + (1-t) * a[i]

    return res

def table_to_colors(table):
    largest = get_largest_element_of_table(table)

    pixels = []

    color1 = [1, 0, 0]
    color2 = [0, 1, 0]

    for row in table:
        pixels.append([])
        for value in row:
            c = [1, 1, 1]
            if value is not None:
                c = mix_colors(color1, color2, value / largest)

            pixels[-1].append(c)

    # last column/row
    for row in pixels:
        row[-1] = [0.7, 0.7, 0.7]

    for cell in pixels[-1]:
        cell[0] = 0.7
        cell[1] = 0.7
        cell[2] = 0.7

    return pixels

    

async def collect_track_price_data(s, series, track_infos, cust_id):
    data = TrackCarData()

    print('Processing {0} series'.format(len(series)))

    parallel_step = 3 
    for i in range(0, len(series), parallel_step):
        session_results = await asyncio.gather(*[get_session_results(s, series[k]['subsession_id']) for k in range(i, min(i+parallel_step, len(series)))])

        for session_result in session_results:
            track_id = get_track_id(session_result)

            track_name = get_track_name(track_infos, track_id)
            car_name = get_car_used_in_session(session_result, cust_id)
            time = get_time_spent_in_session(session_result, cust_id)

            data.add_data(track_name, car_name, time)

    track_labels, car_labels, table = data.to_table()

    fig, ax = plt.subplots(figsize=(10,10))
    im = ax.imshow(table_to_colors(table))

    ax.set_xticks(np.arange(len(car_labels)))
    ax.set_yticks(np.arange(len(track_labels)))

    ax.set_xticklabels(car_labels, fontsize=4)
    ax.set_yticklabels(track_labels, fontsize=4)

    plt.setp(ax.get_xticklabels(), rotation=90, ha='right', rotation_mode='anchor')

    for i in range(len(track_labels)):
        for j in range(len(car_labels)):
            v = table[i][j]
            if v is not None:
                ax.text(j, i, '{0:.1f}'.format(v), ha='center', va='center', color='w', fontsize=2)

    fig.tight_layout()

    # plt.show()
    plt.savefig('figure.png', dpi=800)


async def auth(s):
    # token created by encode_pw
    user = os.getenv('IRACING_USER')
    token = os.getenv('IRACING_TOKEN')

    async with s.post(BASEURL + '/auth', data={'email': user, 'password': token}) as res:
        if res.status != 200:
            raise 'auth error'

async def legacy_main(driver_name):
    async with aiohttp.ClientSession() as s:
        await auth(s)
        cust_id = await get_cust_id(s, driver_name)
        track_infos = await get_track_infos(s)
        member_since = await get_member_since(s, cust_id)
        member_since_year = int(member_since[0:4])

        series = []

        for year in range(member_since_year, 2022+1):
            for quarter in range(1, 4+1):
                print('Querying {0}s{1}'.format(year, quarter))
                series += await search_series(s, cust_id, year, quarter)

        # collect_cumulative_data(s, series, track_infos, cust_id)
        await collect_track_price_data(s, series, track_infos, cust_id)

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
    cur.execute(
        '''CREATE TABLE session(
            session_id INTEGER UNIQUE,
            series_name TEXT
        )'''
    )
    cur.execute(
        '''CREATE TABLE subsession(
            subsession_id INTEGER UNIQUE,
            session_id INTEGER,
            start_time INTEGER,
            license_category_id INTEGER
        )'''
    )
    cur.execute(
        '''CREATE TABLE driver_result(
            cust_id INTEGER,
            team_id INTEGER,
            subsession_id INTEGER,
            simsession_number INTEGER,
            newi_rating INTEGER,
            UNIQUE(cust_id, team_id, subsession_id, simsession_number)
        )'''
    )
    cur.execute(
        '''CREATE TABLE simsession(
            subsession_id INTEGER,
            simsession_number INTEGER,
            simsession_type INTEGER,
            UNIQUE(subsession_id, simsession_number)
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
            ?  /* newi_rating */
        )''', (
            cust_id,
            team_id,
            subsession_id,
            simsession_number,
            driver_result['newi_rating']
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
            ?  /* license_category_id */
        )''', (
            subsession_id,
            subsession['session_id'],
            parse_date(subsession['start_time']),
            subsession['license_category_id']
        )
    )

    add_session_to_db(cur, subsession)

    for simsession in subsession['session_results']:
        add_simsession_to_db(cur, subsession_id, simsession)

def query_db(driver_name):
    con = sqlite3.connect(SQLITE_DB_FILE)
    cur = con.cursor()

    rows = cur.execute(
        '''SELECT subsession.start_time, driver_result.newi_rating FROM
            driver_result
            JOIN simsession ON
                driver_result.subsession_id = simsession.subsession_id AND
                driver_result.simsession_number = simsession.simsession_number
            JOIN subsession ON
                simsession.subsession_id = subsession.subsession_id
            WHERE
                driver_result.cust_id = (SELECT cust_id FROM driver WHERE display_name = ?) AND
                driver_result.newi_rating != -1 AND
                subsession.license_category_id = 2
            ORDER BY subsession.start_time ASC;
        ''', (driver_name,)
    )

    for row in rows:
        start_time = row[0]
        irating = row[1]
        print('{0},{1}'.format(start_time, irating))


def rebuild_db():
    os.remove(SQLITE_DB_FILE)

    con = sqlite3.connect(SQLITE_DB_FILE)
    cur = con.cursor()
    build_db_schema(cur)

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

    con.commit()


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('-r', '--rebuild', action='store_true')
    parser.add_argument('-q', '--query')
    parser.add_argument('-l', '--legacy')

    args = parser.parse_args()

    if args.legacy:
        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)
        try:
            loop.run_until_complete(legacy_main(args.legacy))
        except KeyboardInterrupt:
            pass
    elif args.query:
        query_db(args.query)
    elif args.rebuild:
        rebuild_db()
