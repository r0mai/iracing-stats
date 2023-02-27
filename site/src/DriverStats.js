import './DriverStats.css'
import { toHours, round } from './Utility.js'

import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableRow from '@mui/material/TableRow';

function calcGlobalStats(sessions, trackMap) {
    let time = 0;
    let distance = 0;
    let laps = 0;
    sessions.forEach(session => {
        time += session["average_lap"] * session["laps_complete"];
        distance += trackMap[session["track_id"]]["track_config_length"] * session["laps_complete"];
        laps += session["laps_complete"];
    });

    return {
        time: time,
        distance: distance,
        laps: laps
    };
}

function DriverStats({driverSessions, trackMap, driverName}) {
    let globalStats = calcGlobalStats(driverSessions, trackMap);
    return (
        <TableContainer sx={{maxWidth:'500px'}}>
            <Table>
                <TableBody>
                    <TableRow>
                        <TableCell>Name:</TableCell>
                        <TableCell><b>{driverName}</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Total laps:</TableCell>
                        <TableCell>{globalStats.laps}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Total time:</TableCell>
                        <TableCell>{round(toHours(globalStats.time), 1) + "h"}</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>Total distance:</TableCell>
                        <TableCell>{round(globalStats.distance, 1) + "km"}</TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </TableContainer>
    );
}

export default DriverStats;