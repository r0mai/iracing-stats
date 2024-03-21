import { useD3 } from './hooks/useD3.js';
import { linePlot } from './Plot.js';
import { isRookie, isRace, isMainEvent, isDateCorrectedCategory, isOfficial } from './Utility.js';

function plotIRatingHistory(div, sessions, trackMap, categoryIndices) {
    let filtered = categoryIndices.map((categoryIdx) => {
        return sessions.filter((session) => {
            return (
                !isRookie(session) &&
                isMainEvent(session) &&
                isRace(session) &&
                isOfficial(session) &&
                isDateCorrectedCategory(session, trackMap, categoryIdx)
                // isTrackCategory(session, trackMap, categoryIdx)
            );
        });
    });

    console.log(filtered);
    if (filtered.every((e) => { return e.length === 0; })) {
        div.innerHTML = "No data";
    } else {
        linePlot(div, filtered, e => e["start_time"], e => e["new_irating"], {
            showHorizontalGridLines: true,
            lineColors: ["red", "green", "blue"]
        });
    }
}

function IRatingHistory({driverSessions, trackMap, categoryIndices}) {
    const ref = useD3(
        (root) => {
            plotIRatingHistory(root, driverSessions, trackMap, categoryIndices);
        },
        [driverSessions, categoryIndices]
    );
    
    return (
        <div ref={ref}/>
    );
}

export default IRatingHistory;