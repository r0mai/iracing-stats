import { useD3 } from './hooks/useD3.js';
import { linePlot } from './Plot.js';

function plotIRatingHistory(div, sessions) {
    let filtered = sessions.filter((session) => {
        return (
            session["new_irating"] !== -1 &&
            session["event_type"] === 5 && // race
            session["simsession_number"] === 0 &&
            session["license_category"] === 2 // road
        );
    });

    linePlot(div, filtered, e => e["start_time"], e => e["new_irating"]);
}

function IRatingHistory({driverSessions}) {
    const ref = useD3(
        (root) => {
            plotIRatingHistory(root, driverSessions);
        },
        [driverSessions]
    );
    
    return <div ref={ref}/>;
}

export default IRatingHistory;