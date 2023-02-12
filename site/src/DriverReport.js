import { useFetch } from "react-async";
import DriverStats from './DriverStats.js'
import IRatingHistory from './IRatingHistory.js'
import { driverToQueryParam } from "./Utility.js";

function preprocessDriverSessions(sessions) {
    sessions.forEach(session => {
        session['start_time'] = new Date(session['start_time']);
    });
    sessions.sort((a, b) => a['start_time'].getTime() - b['start_time'].getTime());
}

function DriverReport({driver}) {
    let driverStatsElement;
    let iratingHistoryElement;

    let driverQueryParam = driverToQueryParam(driver);

    let headers = { Accept: "application/json" }
    {
        let { data, error, isPending, run } = useFetch("/api/v1/driver-stats?" + driverQueryParam, {headers});

        let driverStats = data;
        if (isPending) {
            driverStatsElement = "...";
        } else if (error) {
            driverStatsElement = `Something went wront: ${error.message}`;
        } else if (driverStats) {
            driverStatsElement = <DriverStats driverStats={driverStats}/>;
        }
    }

    {
        let { data, error, isPending, run } = useFetch("/api/v1/driver-sessions?" + driverQueryParam, {headers});

        let driverSessions = data;
        if (isPending) {
            iratingHistoryElement = "...";
        } else if (error) {
            iratingHistoryElement = `Something went wront: ${error.message}`;
        } else if (driverSessions) {
            preprocessDriverSessions(driverSessions);
            iratingHistoryElement = <IRatingHistory driverSessions={driverSessions}/>;
        }
    }

    return (
        <div>
            {driverStatsElement}
            {iratingHistoryElement}
        </div>
    );
}

export default DriverReport;