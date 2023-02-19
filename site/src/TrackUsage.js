import { useD3 } from './hooks/useD3.js';
import { verticalBarChart } from './Plot.js';
import { toHours, round, mapifyCarData, mapifyTrackData } from './Utility.js';

function collectTrackUsage(sessions, trackCarData) {
    let trackMap = mapifyTrackData(trackCarData["tracks"]);

    let trackUsage = {};
    sessions.forEach(session => {
        let trackId = session["track_id"];
        let packageId = session["package_id"];
        if (!trackUsage[packageId]) {
            trackUsage[packageId] = {
                track_name: trackMap[trackId]["track_name"],
                time: 0,
                distance: 0.0
            };
        }

        trackUsage[packageId].time += session["average_lap"] * session["laps_complete"];
        trackUsage[packageId].distance += trackMap[trackId]["track_config_length"] * session["laps_complete"];
    });

    return Object.values(trackUsage);
}

function plotTrackUsageTime(div, trackUsage) {
    let format = {
        xTickFormat: (e => round(e, 1) + "h"),
        barFill: "#6EB5FF"
    };
    trackUsage.sort((lhs, rhs) => rhs["time"] - lhs["time"]);
    verticalBarChart(
        div,
        trackUsage,
        e => toHours(e["time"]),
        e => e["track_name"],
        format
    );
}


function TrackUsage({driverSessions, trackCarData}) {
    let trackUsage = collectTrackUsage(driverSessions, trackCarData);
    const ref = useD3(
        (root) => {
            plotTrackUsageTime(root, trackUsage);
        },
        [driverSessions, trackCarData]
    );
    
    return <div ref={ref}/>;
}

export default TrackUsage;