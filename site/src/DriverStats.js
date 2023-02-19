import './DriverStats.css'
import { toHours, round, mapifyTrackData } from './Utility.js'

function calcGlobalStats(sessions, trackMap) {
    let time = 0;
    let distance = 0;
    let laps = 0;
    sessions.forEach(session => {
        time += session["average_lap"] * session["laps_complete"];
        distance += trackMap[session["track_id"]]["track_config_length"] * session["laps_complete"];
        laps += session["laps_complete"];
    });

    return {
        time: time,
        distance: distance,
        laps: laps
    };
}

function DriverStats({driverSessions, trackMap, driverName}) {
    let globalStats = calcGlobalStats(driverSessions, trackMap);
    return (
        <table class="driver-stats-table">
            <tbody>
                <tr>
                    <td>Name:</td>
                    <td>{driverName}</td>
                </tr>
                <tr>
                    <td>Licenses:</td>
                    <td>...licenses...</td>
                </tr>
                <tr>
                    <td>Total laps:</td>
                    <td>{globalStats.laps}</td>
                </tr>
                <tr>
                    <td>Total time:</td>
                    <td>{round(toHours(globalStats.time), 1) + "h"}</td>
                </tr>
                <tr>
                    <td>Total distance:</td>
                    <td>{round(globalStats.distance, 1) + "km"}</td>
                </tr>
            </tbody>
        </table>
    );
}

export default DriverStats;