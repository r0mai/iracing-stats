import { useFetch } from "react-async";
import DriverStats from './DriverStats.js'
import IRatingHistory from './IRatingHistory.js'

function DriverReport({driver}) {
    let driverStatsElement;
    let iratingHistoryElement;

    let headers = { Accept: "application/json" }
    {
        let { data, error, isPending, run } = useFetch("/api/v1/driver-stats?driver_name=" + driver, {headers});

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
        let { data, error, isPending, run } = useFetch("/api/v1/irating-history?driver_name=" + driver, {headers});

        let iratingHistory = data;
        if (isPending) {
            iratingHistoryElement = "...";
        } else if (error) {
            iratingHistoryElement = `Something went wront: ${error.message}`;
        } else if (iratingHistory) {
            iratingHistoryElement = <IRatingHistory iratingHistory={iratingHistory}/>;
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