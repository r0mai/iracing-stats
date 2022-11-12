function populateIratingHistoryDate(dateDiv, data) {
    graphData = {
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
    graphData = {
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

    console.log(result)
}
