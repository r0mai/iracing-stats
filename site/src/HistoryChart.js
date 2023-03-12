import { useD3 } from './hooks/useD3.js';
import { yearlyFrequencyMap } from './Plot.js';
import { getTimeInSession, round, toHours } from './Utility.js';

function HistoryChart({driverSessions}) {
    const ref = useD3(
        (root) => {
            yearlyFrequencyMap(
                root,
                driverSessions,
                e => e["start_time"],
                e => getTimeInSession(e),
                e => round(toHours(e), 1) + "h");
        },
        [driverSessions]
    );
    
    return <div ref={ref}/>;
}

export default HistoryChart;