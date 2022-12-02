import os
import sys
import asyncio
import aiohttp
import hashlib
import base64
import json
import argparse
import zipfile

from common import *
from db import *

async def get_with_retry(s, url, params):
    while True:
        async with s.get(url, params=params) as res:
            if res.status == 429: # we get rate limited
                print('Rate limited on {0}, sleep 5 seconds'.format(url))
                await asyncio.sleep(5)
            elif res.status == 200:
                return await res.text()
            else:
                print('Request {0} {1} failed'.format(url, params), res)
                raise 'error'


async def get_json(s, url, params):
    return json.loads(await get_with_retry(s, url, params))

async def get_and_read(s, suffix, params):
    res = await get_json(s, BASEURL + suffix, params)
    return await get_json(s, res['link'], {})

async def get_and_read_chunked(s, suffix, params):
    res = await get_json(s, BASEURL + suffix, params)
    chunk_info = res['data']['chunk_info']
    base_url = chunk_info['base_download_url']
    result_array = []
    for file in chunk_info['chunk_file_names']:
        url = base_url + file
        result_array += await get_json(s, url, {})

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

def get_session_cache_path(subsession_id):
    return os.path.join(SESSIONS_DIR, '{0}.session.zip'.format(subsession_id))

def is_session_cached(subsession_id):
    return os.path.exists(get_session_cache_path(subsession_id))

async def sync_subsession(s, subsession_id, prefix=''):
    if is_session_cached(subsession_id):
        return

    cached_path = get_session_cache_path(subsession_id)

    print('{0}Syncing session {1}'.format(prefix, subsession_id))
    result = await get_and_read(s, '/data/results/get/', {'subsession_id': subsession_id})

    with zipfile.ZipFile(cached_path, 'w') as zip:
        zip.writestr('session.json', json.dumps(result))

async def sync_tracks_infos(s):
    data = await get_and_read(s, '/data/track/get', {})
    with open(TRACK_DATA_FILE, 'w') as file:
        json.dump(data, file)

async def sync_car_infos(s):
    data = await get_and_read(s, '/data/car/get', {})
    with open(CAR_DATA_FILE, 'w') as file:
        json.dump(data, file)

def encode_pw(username, password):
    initialHash = hashlib.sha256((password + username.lower()).encode('utf-8')).digest()
    hashInBase64 = base64.b64encode(initialHash).decode('utf-8')
    return hashInBase64

async def auth(s):
    # token created by encode_pw
    user = os.getenv('IRACING_USER')
    token = os.getenv('IRACING_TOKEN')

    async with s.post(BASEURL + '/auth', data={'email': user, 'password': token}) as res:
        if res.status != 200:
            raise 'auth error'

async def find_subsessions_for_driver(s, cust_id):
    member_since = await get_member_since(s, cust_id)
    member_since_year = int(member_since[0:4])

    series = []

    for year in range(member_since_year, 2022+1):
        for quarter in range(1, 4+1):
            print('Querying {0}s{1}'.format(year, quarter))
            series += await search_series(s, cust_id, year, quarter)

    return [ses['subsession_id'] for ses in series]

async def sync_subsessions(s, subsession_ids):
    count = len(subsession_ids)
    print('Syncing {0} subsessions'.format(count))
    parallel_step = 3 
    for i in range(0, count, parallel_step):
        await asyncio.gather(*[sync_subsession(s, subsession_ids[k], '{0}/{1} '.format(k, count))
            for k in range(i, min(i+parallel_step, count))])


async def find_non_cached_subsessions_for_driver(s, cust_id):
    subsessions = await find_subsessions_for_driver(s, cust_id)

    non_cached = [] 
    for subsession_id in subsessions:
        if not is_session_cached(subsession_id):
            non_cached.append(subsession_id)

    print('Non-cached sessions {0}/{1}'.format(len(non_cached), len(subsessions)))
    return non_cached

async def sync_driver(s, driver_name):
    cust_id = await get_cust_id(s, driver_name)
    subsessions = await find_non_cached_subsessions_for_driver(s, cust_id)
    await sync_subsessions(s, subsessions)

async def sync_driver_to_db(s, driver_name):
    cust_id = await get_cust_id(s, driver_name)
    subsessions = await find_non_cached_subsessions_for_driver(s, cust_id)
    await sync_subsessions(s, subsessions)

    con = sqlite3.connect(SQLITE_DB_FILE)
    cur = con.cursor()

    for subsession_id in subsessions:
        session_file = get_session_cache_path(subsession_id)
        with zipfile.ZipFile(session_file, 'r') as zip:
            data = zip.read(zip.namelist()[0])
            add_subsession_to_db(cur, data)

    con.commit()


async def sync_stuff_main(args):
    async with aiohttp.ClientSession() as s:
        await auth(s)

        if args.sync_tracks:
            await sync_tracks_infos(s)
        if args.sync_cars:
            await sync_car_infos(s)
        if args.sync_driver:
            await sync_driver(s, args.sync_driver)
        if args.sync_driver_to_db:
            await sync_driver_to_db(s, args.sync_driver_to_db)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-r', '--rebuild-db', action='store_true')
    parser.add_argument('-u', '--update-db', action='store_true', help='Store cached sessions not in db')
    parser.add_argument('-q', '--query')
    parser.add_argument('-t', '--sync-tracks', action='store_true')
    parser.add_argument('-c', '--sync-cars', action='store_true')
    parser.add_argument('-d', '--sync-driver')
    parser.add_argument('-D', '--sync-driver-to-db')

    args = parser.parse_args()

    if args.rebuild_db:
        rebuild_db()
        return

    if args.update_db:
        update_db()
        return

    if args.query:
        print(query_track_car_usage_matrix(args.query))

    

    if (args.sync_tracks or args.sync_cars or
        args.sync_driver or args.sync_driver_to_db):

        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)
        loop.run_until_complete(sync_stuff_main(args))


if __name__ == '__main__':
    main()
