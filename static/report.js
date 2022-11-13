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

async function updateIratingHistory(dateDiv, raceDiv, driverName) {
    let resp = await fetch('/api/v1/irating-history?driver_name=' + driverName);
    let result = await resp.json()

    populateIratingHistoryDate(dateDiv, result);
    populateIratingHistoryRace(raceDiv, result);
}

async function updateCarTrackUsageStats(div, driverName) {
    let resp = await fetch('/api/v1/car-track-usage-stats?driver_name=' + driverName);
    let result = await resp.json()

    var graphData = {
        x: [],
        y: [],
        z: [],
        type: 'heatmap'

    };

    graphData.x = result.cars;
    graphData.y = result.tracks;

    graphData.z = Array.from(Array(graphData.y.length), () => new Array(graphData.x.length));

    for (c = 0; c < result.cars.length; ++c) {
        for (t = 0; t < result.tracks.length; ++t) {
            var r = result.matrix[t][c];
            graphData.z[t][c] = r['time'];
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


    Plotly.newPlot(div, [graphData], layout);
}
