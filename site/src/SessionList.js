import { DataGrid } from '@mui/x-data-grid';
import './SessionList.css';
import * as Category from './LicenseCategory.js';

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
                "start_time": session["start_time"],
                "series_name": session["series_name"],
                "car": carMap[session["car_id"]]["car_name"],
                "track": trackMap[session["track_id"]]["track_name"],
                "irating_delta": session["new_irating"] - session["old_irating"],
                "finish_position_in_class": session["finish_position_in_class"] + 1,
                "new_irating": session["new_irating"],
                "license_category": Category.toNiceName(session["license_category"]),
                "track_category": Category.toNiceName(trackMap[session["track_id"]]["category"]),
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
            field: "start_time",
            headerName: "Date",
            width: 200,
            valueFormatter: params => {
                return params.value.toLocaleString();
            }
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
            field: "finish_position_in_class",
            headerName: "Pos",
            width: 60,
        },
        {
            field: "irating_delta",
            headerName: "IR Δ",
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
        },
        {
            field: "new_irating",
            headerName: "New IR",
            width: 60,
        },
        // {
        //     field: "license_category",
        //     headerName: "Cat",
        //     width: 100
        // },
        // {
        //     field: "track_category",
        //     headerName: "Track Cat",
        //     width: 100
        // },
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