import { useD3 } from './hooks/useD3.js';
import { linePlot } from './Plot.js';
import { isRookie, isRace, isMainEvent, isCategory } from './Utility.js';
import * as Category from './LicenseCategory.js';

function plotIRatingHistory(div, sessions, category) {
    let categoryIdx = Category.findIndex(category);
    let filtered = sessions.filter((session) => {
        return (
            !isRookie(session) &&
            isMainEvent(session) &&
            isRace(session) &&
            isCategory(session, categoryIdx)
        );
    });

    if (filtered.length === 0) {
        div.innerHTML = "No data";
    } else {
        linePlot(div, filtered, e => e["start_time"], e => e["new_irating"], {});
    }
}

function IRatingHistory({driverSessions, category}) {
    const ref = useD3(
        (root) => {
            plotIRatingHistory(root, driverSessions, category);
        },
        [driverSessions, category]
    );
    
    return (
        <div ref={ref}/>
    );
}

export default IRatingHistory;