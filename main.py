import os
import sys
import requests
import hashlib
import base64
import json
import matplotlib.pyplot as plt
import numpy as np

BASEURL = 'https://members-ng.iracing.com'

def get(s, suffix, params):
    res = s.get(BASEURL + suffix, params=params)
    if res.status_code != 200:
        print('Request {0} {1} failed'.format(suffix, params), res)
        print(res.json())
        raise 'error'
    return res

def get_json(s, suffix, params):
    return get(s, suffix, params).json()

def get_and_read(s, suffix, params):
    res = get_json(s, suffix, params)
    return s.get(res['link']).json()

def get_and_read_chunked(s, suffix, params):
    res = get_json(s, suffix, params)
    chunk_info = res['data']['chunk_info']
    base_url = chunk_info['base_download_url']
    result_array = []
    for file in chunk_info['chunk_file_names']:
        url = base_url + file
        result_array += s.get(url).json()

    return result_array

def get_cust_id(s, search_term):
    res = get_and_read(s, '/data/lookup/drivers', {'search_term': search_term})
    if len(res) != 1:
        print('Driver not found')
        raise 'Not found'
    return res[0]['cust_id']

def search_series(s, cust_id, year, quarter):
    return get_and_read_chunked(s, '/data/results/search_series', {
        'cust_id': cust_id,
        'season_year': year,
        'season_quarter': quarter
    })

def get_session_results(s, subsession_id):
    cached_path = 'sessions/{0}.session'.format(subsession_id)
    if os.path.exists(cached_path):
        with open(cached_path, 'r') as file:
            return json.load(file)

    result = get_and_read(s, '/data/results/get/', {'subsession_id': subsession_id})

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

def get_track_infos(s):
    data = get_and_read(s, '/data/track/get', {})

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

def collect_cumulative_data(s, series, track_infos, cust_id):
    time_spent = 0
    length_driven = 0

    for ser in series:
        session_result = get_session_results(s, ser['subsession_id'])

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

    hours = time_spent / 10000 / 60 / 60
    print('Time spent: {0:.1f} hours'.format(hours))
    print('Length driven: {0:.1f}km'.format(length_driven))
    print('Average speed: {0:.1f}km/h'.format(length_driven / hours))


class TrackCarData:
    def __init__(self):
        self._track_set = set()
        self._car_set = set()

        # data[track_name][car_name]
        self.data = dict()

    def add_data(self, track_name, car_name, data):
        self._ensure_track(track_name)
        self._ensure_car(car_name)
        if self.data[track_name][car_name] is not None:
            self.data[track_name][car_name] += data
        else:
            self.data[track_name][car_name] = data

    def _ensure_track(self, track_name):
        if track_name not in self._track_set:
            self._track_set.add(track_name)
            self.data[track_name] = dict.fromkeys(self._car_set, None)

    def _ensure_car(self, car_name):
        if car_name not in self._car_set:
            self._car_set.add(car_name)
            for track_name, cars in self.data.items():
                cars[car_name] = None 

    def to_table(self):
        car_indices = dict()
        track_indices = dict()

        for track_name in self.data.keys():
            track_indices[track_name] = len(track_indices)

        for car_name in list(self.data.values())[0].keys():
            car_indices[car_name] = len(car_indices)

        table = []
        for i in range(0, len(track_indices)):
            table.append([None] * len(car_indices))

        for track, cars in self.data.items():
            for car, time in cars.items():
                if time is not None:
                    table[track_indices[track]][car_indices[car]] = time / 10000 / 60 / 60

        car_labels = [''] * len(car_indices)
        for car_name, idx in car_indices.items():
            car_labels[idx] = car_name

        track_labels = [''] * len(track_indices)
        for track_name, idx in track_indices.items():
            track_labels[idx] = track_name


        return track_labels, car_labels, table

def get_largest_element_of_table(table):
    largest = 0
    for a in table:
        for b in a:
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

    return pixels

    

def collect_track_price_data(s, series, track_infos, cust_id):
    data = TrackCarData()

    for ser in series:
        session_result = get_session_results(s, ser['subsession_id'])

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

    ax.set_xticklabels(car_labels, fontsize=6)
    ax.set_yticklabels(track_labels, fontsize=6)

    plt.setp(ax.get_xticklabels(), rotation=90, ha='right', rotation_mode='anchor')

    for i in range(len(track_labels)):
        for j in range(len(car_labels)):
            v = table[i][j]
            if v is not None:
                ax.text(j, i, '{0:.1f}'.format(v), ha='center', va='center', color='w', fontsize=4)

    fig.tight_layout()

    # plt.show()
    plt.savefig('figure.png', dpi=800)


def auth(s):
    # token created by encode_pw
    user = os.getenv('IRACING_USER')
    token = os.getenv('IRACING_TOKEN')

    res = s.post(BASEURL + '/auth', data={'email': user, 'password': token})
    if res.status_code != 200:
        raise 'auth error'


if __name__ == '__main__':
    s = requests.Session()
    auth(s)
    cust_id = get_cust_id(s, sys.argv[1])
    track_infos = get_track_infos(s)

    time_spent = 0
    length_driven = 0

    series = []

    for year in range(2022, 2022+1):
        for quarter in range(1, 4+1):
            series += search_series(s, cust_id, year, quarter)

    # collect_cumulative_data(s, series, track_infos, cust_id)
    collect_track_price_data(s, series, track_infos, cust_id)
