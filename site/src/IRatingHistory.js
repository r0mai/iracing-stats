import { useD3 } from './hooks/useD3.js';
import { linePlot } from './Plot.js';

function plotIRatingHistory(div, sessions, category) {
    let filtered = sessions.filter((session) => {
        return (
            session["new_irating"] !== -1 &&
            session["event_type"] === 5 && // race
            session["simsession_number"] === 0 &&
            session["license_category"] === category 
        );
    });

    linePlot(div, filtered, e => e["start_time"], e => e["new_irating"]);
}

function IRatingHistory({driverSessions, category}) {
    const ref = useD3(
        (root) => {
            plotIRatingHistory(root, driverSessions, category);
        },
        [driverSessions]
    );
    
    return (
        <div ref={ref}/>
    );
}

export default IRatingHistory;