import os
import sys
import requests
import hashlib
import base64

BASEURL = 'https://members-ng.iracing.com'

def get(s, suffix, params):
    res = s.get(BASEURL + suffix, params=params)
    if res.status_code != 200:
        print('Request {0} {1} failed'.format(suffix, params), res)
        print(res.json())
        raise 'error'
    return res.json()

def get_and_read(s, suffix, params):
    res = get(s, suffix, params)
    return s.get(res['link']).json()

def get_and_read_chunked(s, suffix, params):
    res = get(s, suffix, params)
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
    print(get_and_read(s, '/results/get/', {'subsession_id': subsession_id}))

def encode_pw(username, password):
    initialHash = hashlib.sha256((password + username.lower()).encode('utf-8')).digest()
    hashInBase64 = base64.b64encode(initialHash).decode('utf-8')
    return hashInBase64


def auth(s):
    # token created by encode_pw
    user = os.getenv('IRACING_USER')
    token = os.getenv('IRACING_TOKEN')

    res = s.post(BASEURL + '/auth', data={'email':user, 'password': token})
    if res.status_code != 200:
        raise 'auth error'


if __name__ == '__main__':
    s = requests.Session()
    auth(s)
    cust_id = get_cust_id(s, sys.argv[1])
    series = search_series(s, cust_id, 2022, 3)
    get_session_results(s, series[0]['subsession_id'])
