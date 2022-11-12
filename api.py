from flask import Flask, request, jsonify
from db import *

app = Flask('iracing-charts')

@app.get('/api/v1/irating-history')
def get_irating_history():
    driver_name = request.args.get('driver_name')
    if not driver_name:
        return 'Need driver_name'

    return query_irating_history(driver_name)

@app.get('/api/v1/car-track-usage-stats')
def get_car_track_usage_stats():
    driver_name = request.args.get('driver_name')
    if not driver_name:
        return 'Need driver_name'

    raw_data = query_track_car_usage_matrix(driver_name)

    car_idxs = dict()
    track_idxs = dict()

    car_idx = 0
    track_idx = 0

    for car_track in raw_data:
        car = car_track['car_name']
        track = car_track['track_name']

        if car not in car_idxs:
            car_idxs[car] = car_idx
            car_idx += 1

        if track not in track_idxs:
            track_idxs[track] = track_idx
            track_idx += 1

    car_count = len(car_idxs)
    track_count = len(track_idxs)


    # matrix[track][car]
    matrix = [[dict() for x in range(car_count)] for y in range(track_count)]

    for car_track in raw_data:
        car = car_track['car_name']
        track = car_track['track_name']

        obj = dict(
            time = car_track['time']
        )
        matrix[track_idxs[track]][car_idxs[car]] = obj

    cars = ['' for x in range(car_count)]
    tracks = ['' for x in range(track_count)]

    for name, idx in car_idxs.items():
        cars[idx] = name

    for name, idx in track_idxs.items():
        tracks[idx] = name

    result = dict(
        matrix = matrix,
        cars = cars,
        tracks = tracks
    )

    return result

if __name__ == '__main__':
    app.run()
