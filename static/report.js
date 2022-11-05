async function updateIratingHistory(div, driverName) {
    console.log('update irating', div, driverName);
    let resp = await fetch('/api/v1/irating-history?driver_name=' + driverName);
    let result = await resp.json()

    graphData = {
        x: [],
        y: [],
        text: [],
        hovertemplate:
            'Date: %{x}<br>' +
            'iRating: %{y}<br>' +
            '%{text}'
    };

    result.forEach(race => {
        graphData.x.push(new Date(race['start_time'] * 1000));
        graphData.y.push(race['irating']);
        graphData.text.push(race['series_name']);
    });

    Plotly.newPlot(iratingHistoryDiv, [graphData], {
        margin: { t: 0 }
    });
}

