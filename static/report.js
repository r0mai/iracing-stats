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

function populateTrackUsageStats(div, data, key, valueMutator) {
    var graphData = {
        x: [],
        y: [],
        z: [],
        type: 'heatmap'

    };


    var car_sums = {};
    var track_sums = {};
    var full_sum = 0;

    for (var c = 0; c < data.cars.length; ++c) {
        for (var t = 0; t < data.tracks.length; ++t) {
            var r = data.matrix[t][c];
            var value = r[key];
            if (!value) {
                continue;
            }

            var car = data.cars[c];
            var track = data.tracks[t];

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
    var car_idxs = [...Array(data.cars.length).keys()];
    var track_idxs = [...Array(data.tracks.length).keys()];

    car_idxs.sort((lhs, rhs) => {
       return (car_sums[data.cars[rhs]] ?? 0) - (car_sums[data.cars[lhs]] ?? 0)
    });

    track_idxs.sort((lhs, rhs) => {
       return (track_sums[data.tracks[lhs]] ?? 0) - (track_sums[data.tracks[rhs]] ?? 0);
    });

    for (var c = 0; c < data.cars.length; ++c) {
        graphData.x[c] = data.cars[car_idxs[c]];
    }

    for (var t = 0; t < data.tracks.length; ++t) {
        graphData.y[t] = data.tracks[track_idxs[t]];
    }

    graphData.z = Array.from(Array(graphData.y.length), () => new Array(graphData.x.length));

    for (var c = 0; c < data.cars.length; ++c) {
        for (var t = 0; t < data.tracks.length; ++t) {
            var r = data.matrix[track_idxs[t]][car_idxs[c]];
            graphData.z[t][c] = valueMutator(r[key]);
        }
    }

    var layout = {
        // width: data.cars.length * 20,
        height: data.tracks.length * 20,
        xaxis: {
            constrain: 'domain'
        },
        yaxis: {
            scaleanchor: 'x'
        }
    };


    Plotly.newPlot(div, [graphData], layout);
}

function populateTrackUsageStackBar(div, data, key, valueMutator) {
    var traces = [];
    for (var t = 0; t < data.tracks.length; ++t) {
        traces[t] = {
            x: [],
            y: [],
            name: data.tracks[t],
            type: 'bar'
        }
    }

    for (var c = 0; c < data.cars.length; ++c) {
        for (var t = 0; t < data.tracks.length; ++t) {
            var r = data.matrix[t][c];
            var value = r[key];
            if (!value) {
                continue;
            }
            var car = data.cars[c];
            var carIdx = traces[t].x.indexOf(car);
            if (carIdx == -1) {
                carIdx = traces[t].x.length;
                traces[t].x.push(car);
                traces[t].y.push(0);
            }
            traces[t].y[carIdx] += valueMutator(value);
        }
    }

    var layout = {barmode: 'stack'};

    Plotly.newPlot(div, traces, layout);
}

async function updateCarTrackUsageStats(divTime, divLaps, divTrackStack, driverName) {
    let resp = await fetch('/api/v1/car-track-usage-stats?driver_name=' + driverName);
    let result = await resp.json()

    populateTrackUsageStats(divTime, result, 'time', toHours);
    populateTrackUsageStats(divLaps, result, 'laps', (x) => { return x; });
    populateTrackUsageStackBar(divTrackStack, result, 'time', toHours);
}
