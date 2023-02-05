import { useFetch } from "react-async";
import DriverStats from './DriverStats.js'

function DriverReport({driver}) {
    let headers = { Accept: "application/json" }
    let { driverStats, error, isPending, run } = useFetch("/api/v1/driver-stats?driver_name=" + driver, {headers});

    if (isPending) {
        return "...";
    }
    if (error) {
        return `Something went wront: ${error.message}`;
    }
    if (driverStats) {
        return <DriverStats driverStats={driverStats}/>;
    }
}

export default DriverReport;