from flask import Flask, request, jsonify
from db import *

app = Flask('iracing-charts')

@app.get('/irating-history')
def get_irating_history():
    driver_name = request.args.get('driver_name')
    if not driver_name:
        return 'Need driver_name'

    result = query_irating_history(driver_name)

    return result

if __name__ == '__main__':
    app.run()
