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

    return query_track_car_usage_matrix(driver_name)

if __name__ == '__main__':
    app.run()
