function populateIratingHistoryDate(dateDiv, data) {
    let iratingData = {
        x: [],
        y: [],
        text: [],
        name: 'iRating',
        type: 'scatter',
        hovertemplate:
            'Date: %{x}<br>' +
            'iRating: %{y}<br>' +
            '%{text}'
    };
    let cpiData = {
        x: [],
        y: [],
        text: [],
        name: 'CPI',
        type: 'scatter',
        yaxis: 'y2',
        hovertemplate:
            'Date: %{x}<br>' +
            'CPI: %{y}<br>' +
            '%{text}'
    };

    data.forEach(race => {
        let start_time = new Date(Date.parse(race['start_time']));
        iratingData.x.push(start_time);
        iratingData.y.push(race['irating']);
        iratingData.text.push(race['series_name']);

        cpiData.x.push(start_time);
        cpiData.y.push(race['cpi']);
        cpiData.text.push(race['series_name']);
    });

    Plotly.newPlot(dateDiv, [iratingData, cpiData], {
        margin: { t: 0 },
        yaxis: {
            title: 'iRating'
        },
        yaxis2: {
            title: 'CPI',
            side: 'right',
            overlaying: 'y',
        },
    });
}

function populateIratingHistoryRace(raceDiv, data) {
    let iratingData = {
        x: [],
        y: [],
        text: [],
        name: 'iRating',
        hovertemplate:
            'Date: %{x}<br>' +
            'iRating: %{y}<br>' +
            '%{text}'
    };
    let cpiData = {
        x: [],
        y: [],
        text: [],
        name: 'CPI',
        yaxis: 'y2',
        hovertemplate:
            'Date: %{x}<br>' +
            'CPI: %{y}<br>' +
            '%{text}'
    };

    let i = 0;
    data.forEach(race => {
        iratingData.x.push(i);
        iratingData.y.push(race['irating']);
        iratingData.text.push(race['series_name']);

        cpiData.x.push(i);
        cpiData.y.push(race['cpi']);
        cpiData.text.push(race['series_name']);

        ++i;
    });

    Plotly.newPlot(raceDiv, [iratingData, cpiData], {
        margin: { t: 0 },
        yaxis: {
            title: 'iRating'
        },
        yaxis2: {
            title: 'CPI',
            side: 'right',
            overlaying: 'y',
        },
    });
}

function svgTransform(w, h) {
    return "translate(" + w + "," + h + ")";
}

function populateIratingHistoryRaceD3JSDiv(raceD3JSDiv, result) {
    // preprocess data: add index field
    result = result.map((d, idx) => ({...d, index: idx}));

    var margin = {top: 10, right: 30, bottom: 30, left: 60},
        width = 800 - margin.left - margin.right,
        height = 400 - margin.top - margin.bottom;

    // append the svg object to the body of the page
    var svg = d3.select(raceD3JSDiv)
        .append("svg")
            .attr("width", width + margin.left + margin.right)
            .attr("height", height + margin.top + margin.bottom)
        .append("g")
            .attr("transform", svgTransform(margin.left, margin.top));
    
    var x = d3.scaleLinear()
        .domain([0, result.length])
        .range([0, width]);

    var y = d3.scaleLinear()
        .domain(d3.extent(result, e => e["irating"]))
        .range([height, 0]);
    
    svg.append("g")
        .attr("transform", svgTransform(0, height))
        .call(d3.axisBottom(x));

    svg.append("g")
        .call(d3.axisLeft(y));

    var line = d3.line()
        .x(d => x(d["index"]))
        .y(d => y(d["irating"]));

    svg.append("path")
        .datum(result)
        .attr("fill", "none")
        .attr("stroke", "red")
        .attr("stroke-width", 1.5)
        .attr("d", line);
}

function toHours(interval) {
    return interval / 10000 / 60 / 60;
}

async function updateIratingHistory(dateDiv, raceDiv, raceD3JSDiv, driverName, category) {
    let resp = await fetch('/api/v1/irating-history?driver_name=' + driverName + "&category=" + category);
    let result = await resp.json()

    populateIratingHistoryDate(dateDiv, result);
    populateIratingHistoryRace(raceDiv, result);
    populateIratingHistoryRaceD3JSDiv(raceD3JSDiv, result);
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

function populateCarUsageStackBar(div, data, key, valueMutator) {
    var traces = [];
    for (var t = 0; t < data.cars.length; ++t) {
        traces[t] = {
            x: [],
            y: [],
            name: data.cars[t],
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
            var track = data.tracks[t];
            var trackIdx = traces[c].x.indexOf(track);
            if (trackIdx == -1) {
                trackIdx = traces[c].x.length;
                traces[c].x.push(track);
                traces[c].y.push(0);
            }
            traces[c].y[trackIdx] += valueMutator(value);
        }
    }

    var layout = {barmode: 'stack'};

    Plotly.newPlot(div, traces, layout);
}

async function updateCarTrackUsageStats(divTime, divLaps, divTrackStack, divCarStack, driverName) {
    let resp = await fetch('/api/v1/car-track-usage-stats?driver_name=' + driverName);
    let result = await resp.json()

    populateTrackUsageStats(divTime, result, 'time', toHours);
    populateTrackUsageStats(divLaps, result, 'laps', (x) => { return x; });
    populateTrackUsageStackBar(divTrackStack, result, 'time', toHours);
    populateCarUsageStackBar(divCarStack, result, 'time', toHours);
}
