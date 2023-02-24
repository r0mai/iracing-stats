import { useD3 } from './hooks/useD3.js';
import { linePlot } from './Plot.js';
import { isRookie, isRace, isMainEvent, isCategory } from './Utility.js';

function plotIncidentHistory(div, sessions, category) {
    let filtered = sessions.filter((session) => {
        return (
            !isRookie(session) &&
            isMainEvent(session) &&
            isRace(session) &&
            isCategory(session, category)
        );
    });

    linePlot(div, filtered, e => e["start_time"], e => e["new_cpi"]);
}

function IncidentHistory({driverSessions, category}) {
    const ref = useD3(
        (root) => {
            plotIncidentHistory(root, driverSessions, category);
        },
        [driverSessions]
    );
    
    return <div ref={ref}/>;
}

export default IncidentHistory;