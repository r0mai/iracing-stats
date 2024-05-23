import { useD3 } from './hooks/useD3.js';
import { linePlot } from './Plot.js';
import { isRookie, isRace, isMainEvent, isOfficial, isDateCorrectedCategory } from './Utility.js';
import * as Category from './LicenseCategory.js';

function plotIncidentHistory(div, sessions, trackMap, category) {
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

    let mutateColor = function(color) {
        let darkenFactor = 0.5;
        let r = (color >> 16) & 0xFF;
        let g = (color >> 8) & 0xFF;
        let b = (color >> 0) & 0xFF;

        r *= darkenFactor;
        g *= darkenFactor;
        b *= darkenFactor;

        r = Math.round(r);
        g = Math.round(g);
        b = Math.round(b);

        return `rgb(${r}, ${g}, ${b})`;
    }

    let safetyRatingLanes = [
        { min: 0, max: 15, color: mutateColor(0x951b1e) },
        { min: 15, max: 22, color: mutateColor(0x906822) },
        { min: 22, max: 35, color: mutateColor(0x968e1d) },
        { min: 35, max: 50, color: mutateColor(0x177c1c) },
        { min: 50, max: 50000, color: mutateColor(0x174189) },
    ];

    if (filtered.length === 0) {
        div.innerHTML = "No data";
    } else {
        linePlot(div, filtered, e => e["start_time"], e => e["new_cpi"], {
            horizontalLanes: safetyRatingLanes,
            lineColors: ["#cc8", "red", "green"],
            legendLabels: ["Road", "Sports Car", "Formula Car"]
        });
    }
}

function IncidentHistory({driverSessions, trackMap, category}) {
    const ref = useD3(
        (root) => {
            plotIncidentHistory(root, driverSessions, trackMap, category);
        },
        [driverSessions, category]
    );
    
    return <div ref={ref}/>;
}

export default IncidentHistory;