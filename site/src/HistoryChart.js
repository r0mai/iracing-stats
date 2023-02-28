import { useD3 } from './hooks/useD3.js';
import { linePlot, yearlyFrequencyMap } from './Plot.js';

function HistoryChart({driverSessions}) {
    const ref = useD3(
        (root) => {
            yearlyFrequencyMap(root, driverSessions, e => e["start_time"]);
        },
        [driverSessions]
    );
    
    return "Under construction";
    return <div ref={ref}/>;
}

export default HistoryChart;