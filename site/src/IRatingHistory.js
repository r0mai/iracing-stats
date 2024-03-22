import { useD3 } from './hooks/useD3.js';
import { linePlot } from './Plot.js';
import { isRookie, isRace, isMainEvent, isDateCorrectedCategory, isOfficial } from './Utility.js';
import * as Category from './LicenseCategory.js';

function plotIRatingHistory(div, sessions, trackMap, category) {
    let categoryIdx = Category.findIndex(category);
    let categoryIndices = [categoryIdx];
    if (categoryIdx === Category.kRoad) {
        categoryIndices = [Category.kRoad, Category.kSportsCar, Category.kFormulaCar];
    }
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

    if (categoryIdx === Category.kRoad) {
        if (filtered[0].length !== 0) {
            for (let i = 1; i <= 2; ++i) {
                if (filtered[i].length !== 0) {
                    filtered[i].unshift(filtered[0][filtered[0].length - 1]);
                }
            }
        }
    }

    if (filtered.every((e) => { return e.length === 0; })) {
        div.innerHTML = "No data";
    } else {
        linePlot(div, filtered, e => e["start_time"], e => e["new_irating"], {
            showHorizontalGridLines: true,
            lineColors: ["red", "green", "blue"],
            legendLabels: ["Road", "Sports Car", "Formula Car"]
        });
    }
}

function IRatingHistory({driverSessions, trackMap, category}) {
    const ref = useD3(
        (root) => {
            plotIRatingHistory(root, driverSessions, trackMap, category);
        },
        [driverSessions, category]
    );
    
    return (
        <div ref={ref}/>
    );
}

export default IRatingHistory;