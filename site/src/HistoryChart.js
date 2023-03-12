import { useD3 } from './hooks/useD3.js';
import { yearlyFrequencyMap } from './Plot.js';
import { formatTime, getTimeInSession } from './Utility.js';

function HistoryChart({driverSessions}) {
    const ref = useD3(
        (root) => {
            yearlyFrequencyMap(
                root,
                driverSessions,
                e => e["start_time"],
                e => getTimeInSession(e),
                e => formatTime(e)
            )
        },
        [driverSessions]
    );
    
    return <div ref={ref}/>;
}

export default HistoryChart;