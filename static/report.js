function populateIratingHistoryDate(dateDiv, data) {
    var graphData = {
        x: [],
        y: [],
        text: [],
        hovertemplate:
            'Date: %{x}<br>' +
            'iRating: %{y}<br>' +
            '%{text}'
    };

    data.forEach(race => {
        graphData.x.push(new Date(race['start_time'] * 1000));
        graphData.y.push(race['irating']);
        graphData.text.push(race['series_name']);
    });

    Plotly.newPlot(dateDiv, [graphData], {
        margin: { t: 0 }
    });
}

function populateIratingHistoryRace(raceDiv, data) {
    var graphData = {
        x: [],
        y: [],
        text: [],
        hovertemplate:
            'Date: %{x}<br>' +
            'iRating: %{y}<br>' +
            '%{text}'
    };

    var i = 0;
    data.forEach(race => {
        graphData.x.push(i++);
        graphData.y.push(race['irating']);
        graphData.text.push(race['series_name']);
    });

    Plotly.newPlot(raceDiv, [graphData], {
        margin: { t: 0 }
    });
}

function toHours(interval) {
    return interval / 10000 / 60 / 60;
}

async function updateIratingHistory(dateDiv, raceDiv, driverName) {
    let resp = await fetch('/api/v1/irating-history?driver_name=' + driverName);
    let result = await resp.json()

    populateIratingHistoryDate(dateDiv, result);
    populateIratingHistoryRace(raceDiv, result);
}

async function updateCarTrackUsageStats(divTime, divLaps, driverName) {
    let resp = await fetch('/api/v1/car-track-usage-stats?driver_name=' + driverName);
    let result = await resp.json()

    var graphData = {
        x: [],
        y: [],
        z: [],
        type: 'heatmap'

    };


    var car_sums = {};
    var track_sums = {};
    var full_sum = 0;

    for (var c = 0; c < result.cars.length; ++c) {
        for (var t = 0; t < result.tracks.length; ++t) {
            var r = result.matrix[t][c];
            var value = r['time'];
            if (!value) {
                continue;
            }

            var car = result.cars[c];
            var track = result.tracks[t];

            if (!(car in car_sums)) {
                car_sums[car] = 0;
            }
            if (!(track in track_sums)) {
                track_sums[track] = 0;
            }

            car_sums[car] += value;
            track_sums[track] += value;

            full_sum += value;
        }
    }

    // [0, 1, 2, ... n-1
    var car_idxs = [...Array(result.cars.length).keys()];
    var track_idxs = [...Array(result.tracks.length).keys()];

    car_idxs.sort((lhs, rhs) => {
       return (car_sums[result.cars[rhs]] ?? 0) - (car_sums[result.cars[lhs]] ?? 0)
    });

    track_idxs.sort((lhs, rhs) => {
       return (track_sums[result.tracks[lhs]] ?? 0) - (track_sums[result.tracks[rhs]] ?? 0);
    });

    for (var c = 0; c < result.cars.length; ++c) {
        graphData.x[c] = result.cars[car_idxs[c]];
    }

    for (var t = 0; t < result.tracks.length; ++t) {
        graphData.y[t] = result.tracks[track_idxs[t]];
    }

    graphData.z = Array.from(Array(graphData.y.length), () => new Array(graphData.x.length));

    for (var c = 0; c < result.cars.length; ++c) {
        for (var t = 0; t < result.tracks.length; ++t) {
            var r = result.matrix[track_idxs[t]][car_idxs[c]];
            graphData.z[t][c] = toHours(r['time']);
        }
    }

    var layout = {
        // width: result.cars.length * 20,
        height: result.tracks.length * 20,
        xaxis: {
            constrain: 'domain'
        },
        yaxis: {
            scaleanchor: 'x'
        }
    };


    Plotly.newPlot(divTime, [graphData], layout);
}
