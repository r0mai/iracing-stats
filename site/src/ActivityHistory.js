import { useD3 } from './hooks/useD3.js';
import { colorsFromThresholds, yearlyFrequencyMap, plotColorInterpolator } from './Plot.js';
import { fromMinutes, fromHours, formatTime, getTimeInSession } from './Utility.js';

function ActivityHistory({driverSessions}) {
    const ref = useD3(
        (root) => {
            if (driverSessions.length === 0) {
                root.innerHTML = "No data";
            } else {
                let thresholds = [
                    fromHours(1),
                    fromHours(3),
                    fromHours(6),
                    fromHours(12)
                ];
                yearlyFrequencyMap(
                    root,
                    driverSessions,
                    e => e["start_time"],
                    getTimeInSession,
                    e => e === undefined ? "No Activity" : formatTime(e),
                    {
                        thresholds: thresholds,
                        thresholdColors: colorsFromThresholds(thresholds, plotColorInterpolator)
                    }
                );
            }
        },
        [driverSessions]
    );
    
    return <div ref={ref}/>;
}

export default ActivityHistory;