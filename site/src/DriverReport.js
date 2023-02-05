import { useFetch } from "react-async";

function DriverReport({driver}) {
    let headers = { Accept: "application/json" }
    let { data, error, isPending, run } = useFetch("/api/v1/driver-stats?driver_name=" + driver, {headers});

    if (isPending) {
        return "...";
    }
    if (error) {
        return `Something went wront: ${error.message}`;
    }
    if (data) {
        return <h2>{data["laps"]} laps</h2>;
    }
}

export default DriverReport;