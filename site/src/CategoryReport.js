import IRatingHistory from './IRatingHistory.js'
import IncidentHistory from './IncidentHistory.js'

function CategoryReport({driverSessions, category}) {
    return (
        <div>
            <IRatingHistory driverSessions={driverSessions} category={category}/>;
            <IncidentHistory driverSessions={driverSessions} category={category}/>;
        </div>
    );
}

export default CategoryReport;