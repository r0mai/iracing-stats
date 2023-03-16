import { useD3 } from './hooks/useD3.js';
import { verticalBarChart } from './Plot.js';
import { formatTime } from './Utility.js';

function collectCarUsage(sessions, trackMap, carMap) {
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
        xTickFormat: formatTime,
        barFill: "#6EB5FF"
    };
    carUsage.sort((lhs, rhs) => rhs["time"] - lhs["time"]);
    verticalBarChart(
        div,
        carUsage,
        e => e["time"],
        e => e["car_name"],
        format
    );
}


function CarUsage({driverSessions, trackMap, carMap}) {
    let carUsage = collectCarUsage(driverSessions, trackMap, carMap);
    const ref = useD3(
        (root) => {
            plotCarUsageTime(root, carUsage);
        },
        [driverSessions, trackMap, carMap]
    );
    
    return <div ref={ref}/>;
}

export default CarUsage;