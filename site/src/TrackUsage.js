import { useD3 } from './hooks/useD3.js';
import { verticalBarChart } from './Plot.js';
import { formatTime } from './Utility.js';

function collectTrackUsage(sessions, trackMap) {
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
        xTickFormat: formatTime,
        barFill: "#6EB5FF"
    };
    trackUsage.sort((lhs, rhs) => rhs["time"] - lhs["time"]);
    verticalBarChart(
        div,
        trackUsage,
        e => e["time"],
        e => e["track_name"],
        format
    );
}


function TrackUsage({driverSessions, trackMap}) {
    let trackUsage = collectTrackUsage(driverSessions, trackMap);
    const ref = useD3(
        (root) => {
            plotTrackUsageTime(root, trackUsage);
        },
        [driverSessions, trackMap]
    );
    
    return <div ref={ref}/>;
}

export default TrackUsage;