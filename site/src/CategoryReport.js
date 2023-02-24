import IRatingHistory from './IRatingHistory.js'
import IncidentHistory from './IncidentHistory.js'
import { isCategory } from './Utility.js';
import { Typography } from '@mui/material';

function getHighestIRating(driverSessions, category) {
    let filteredIRating = driverSessions.filter(session => isCategory(session, category)).map(session => session["new_irating"]);
    return Math.max.apply(null, filteredIRating);
}

function CategoryReport({driverSessions, category}) {
    return (
        <div>
            <Typography>
                Highest IRating achieved: {getHighestIRating(driverSessions, category)}
            </Typography>
            <IRatingHistory driverSessions={driverSessions} category={category}/>
            <IncidentHistory driverSessions={driverSessions} category={category}/>
        </div>
    );
}

export default CategoryReport;