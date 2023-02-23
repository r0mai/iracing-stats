import { DataGrid } from '@mui/x-data-grid';
import Box from '@mui/material/Box';
import './SessionList.css';

function preprocessSessions(driverSessions, trackMap, carMap) {
    return (driverSessions 
        .filter(session => {
            return (
                session["event_type"] === 5 && // is race
                session["simsession_number"] === 0 &&
                true
            );
        })
        .map(session => {
            return {
                "id": session["subsession_id"],
                "series_name": session["series_name"],
                "car": carMap[session["car_id"]]["car_name"],
                "track": trackMap[session["track_id"]]["track_name"],
                "irating_delta": session["new_irating"] - session["old_irating"],
            };
        })
        .reverse()
    );
}

function SessionList({driverSessions, trackMap, carMap}) {
    let rows = preprocessSessions(driverSessions, trackMap, carMap);
    let columns = [
        {
            field: "id",
            headerName: "Session ID",
            width: 90,
            renderCell: params => {
                return (
                    <a href={"https://members.iracing.com/membersite/member/EventResult.do?&subsessionid=" + params.value}>
                        {params.value}
                    </a>
                );
            },
        },
        {
            field: "series_name",
            headerName: "Series",
            width: 200,
        },
        {
            field: "car",
            headerName: "Car",
            width: 200,
        },
        {
            field: "track",
            headerName: "Track",
            width: 200,
        },
        {
            field: "irating_delta",
            headerName: "IR",
            width: 60,
            valueFormatter: params => {
                return (params.value > 0 ? "+" : "") + params.value;
            },
            cellClassName: params => {
                if (params.value > 0) {
                    return 'positive-gain';
                } else if (params.value < 0) { 
                    return 'negative-gain';
                } else {
                    return '';
                }
            },
        }
    ];
    return (
        <div style={{ width: "100%", height: 660 }}>
            <div style={{ display: 'flex', height: "100%" }}>
                <div style={{ flexGrow: 1 }}>
                    <DataGrid
                        rows={rows}
                        columns={columns}
                        pageSize={10}
                        rowsPerPageOptions={[10]}
                        disableSelectionOnClick
                        autoHeight={true}
                    />
                </div>
            </div>
        </div>
    );
}

export default SessionList;