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
                        // https://colorbrewer2.org/#type=sequential&scheme=OrRd&n=5
                        thresholdColors: ['#fef0d9','#fdcc8a','#fc8d59','#e34a33','#b30000']
                    }
                );
            }
        },
        [driverSessions]
    );
    
    return <div ref={ref}/>;
}

export default ActivityHistory;