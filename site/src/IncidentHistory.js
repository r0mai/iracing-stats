import { useD3 } from './hooks/useD3.js';
import { linePlot } from './Plot.js';

function plotIncidentHistory(div, sessions) {
    let filtered = sessions.filter((session) => {
        return (
            session["new_irating"] !== -1 &&
            session["event_type"] === 5 && // race
            session["simsession_number"] === 0 &&
            session["license_category"] === 2 // road
        );
    });

    linePlot(div, filtered, e => e["start_time"], e => e["new_cpi"]);
}

function IncidentHistory({driverSessions}) {
    const ref = useD3(
        (root) => {
            plotIncidentHistory(root, driverSessions);
        },
        [driverSessions]
    );
    
    return <div ref={ref}/>;
}

export default IncidentHistory;