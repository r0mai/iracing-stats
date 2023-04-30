import { useD3 } from './hooks/useD3.js';
import { linePlot } from './Plot.js';
import { isRookie, isRace, isMainEvent, isCategory } from './Utility.js';
import * as Category from './LicenseCategory.js';

function plotIncidentHistory(div, sessions, category) {
    let categoryIdx = Category.findIndex(category);
    let filtered = sessions.filter((session) => {
        return (
            !isRookie(session) &&
            isMainEvent(session) &&
            isRace(session) &&
            isCategory(session, categoryIdx)
        );
    });

    let safetyRatingLanes = [
        { min: 0, max: 15, color: "#951b1e" },
        { min: 15, max: 22, color: "#906822" },
        { min: 22, max: 35, color: "#968e1d" },
        { min: 35, max: 50, color: "#177c1c" },
        { min: 50, max: 50000, color: "#174189" },
    ];

    if (filtered.length === 0) {
        div.innerHTML = "No data";
    } else {
        linePlot(div, filtered, e => e["start_time"], e => e["new_cpi"], {
            horizontalLanes: safetyRatingLanes,
            lineColor: "#DDD"
        });
    }
}

function IncidentHistory({driverSessions, category}) {
    const ref = useD3(
        (root) => {
            plotIncidentHistory(root, driverSessions, category);
        },
        [driverSessions, category]
    );
    
    return <div ref={ref}/>;
}

export default IncidentHistory;