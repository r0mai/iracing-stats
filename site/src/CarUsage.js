import { useD3 } from './hooks/useD3.js';
import { verticalBarChart } from './Plot.js';
import { toHours, round } from './Utility.js';

function collectCarUsage(sessions, trackCarData) {
    // TODO factor this out
    let trackMap = {};
    trackCarData["tracks"].forEach(track => {
        trackMap[track["track_id"]] = track;
    });

    let carMap = {};
    trackCarData["cars"].forEach(car => {
        carMap[car["car_id"]] = car;
    });

    let carUsage = {};
    sessions.forEach(session => {
        let carId = session["car_id"];
        if (!carUsage[carId]) {
            carUsage[carId] = {
                car_name: carMap[carId]["car_name"],
                car_name_abbreviated: carMap[carId]["car_name_abbreviated"],
                time: 0,
                distance: 0.0
            };
        }

        carUsage[carId].time += session["average_lap"] * session["laps_complete"];
        carUsage[carId].distance += trackMap[session["track_id"]]["track_config_length"] * session["laps_complete"];
    });

    return Object.values(carUsage);
}

function plotCarUsageTime(div, carUsage) {
    let format = {
        xTickFormat: (e => round(e, 1) + "h"),
        barFill: "#6EB5FF"
    };
    carUsage.sort((lhs, rhs) => rhs["time"] - lhs["time"]);
    verticalBarChart(
        div,
        carUsage,
        e => toHours(e["time"]),
        e => e["car_name"],
        format
    );
}


function CarUsage({driverSessions, trackCarData}) {
    let carUsage = collectCarUsage(driverSessions, trackCarData);
    const ref = useD3(
        (root) => {
            plotCarUsageTime(root, carUsage);
        },
        [driverSessions, trackCarData]
    );
    
    return <div ref={ref}/>;
}

export default CarUsage;