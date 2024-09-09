import { DataGrid } from '@mui/x-data-grid';
import './SessionList.css';
import * as Category from './LicenseCategory.js';
import { calcSessionCPI, isRace, round } from './Utility';

function formatStartTime(session) {
    let year = session["season_year"];
    // default year is 2000 for 'non-seasoned' sesssions
    if (year === 2000) {
        return session["start_time"].toLocaleString();
    }
    return session["start_time"].toLocaleString() + " [" + (year - 2000) + "S" + session["season_quarter"] + "]";
}

function preprocessSessions(driverSessions, trackMap, carMap) {
    return (driverSessions 
        .filter(session => {
            return (
                isRace(session) &&
                true
            );
        })
        .map(session => {
            return {
                "id": session["subsession_id"],
                "simsession_number": session["simsession_number"],
                "start_time": formatStartTime(session),
                // hosted sessions have session_name, for official ones we use series_name
                "series_name": session["session_name"] || session["series_name"],
                "car": carMap[session["car_id"]]["car_name"],
                "track": trackMap[session["track_id"]]["track_name"],
                "irating_delta": session["new_irating"] - session["old_irating"],
                "finish_position_in_class": session["finish_position_in_class"] + 1,
                "new_irating": session["new_irating"],
                "license_category": Category.toNiceName(session["license_category"]),
                "track_category": Category.toNiceName(trackMap[session["track_id"]]["category"]),
                "incidents": session["incidents"],
                "cpi": calcSessionCPI(session, trackMap)
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
                    <a href={"https://members-ng.iracing.com/racing/results-stats/results?subsessionid=" + params.value}>
                        {params.value}
                    </a>
                );
            },
        },
        {
            field: "start_time",
            headerName: "Date",
            width: 220,
        },
        {
            field: "series_name",
            headerName: "Series",
            width: 200,
        },
        {
            field: "license_category",
            headerName: "Category",
            width: 100
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
            field: "incidents",
            headerName: "Inc",
            width: 60,
        },
        {
            field: "cpi",
            headerName: "CPI",
            width: 80,
            valueFormatter: params => {
                return isFinite(params.value) ? round(params.value, 1) : "∞";
            }
        },
        {
            field: "irating_delta",
            headerName: "IR Δ",
            width: 80,
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
            width: 80,
        },
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
                        rowsPerPageOptions={[20]}
                        disableSelectionOnClick
                        autoHeight={true}
                        getRowId={(row) => row["id"] + "_" + row["simsession_number"]}
                    />
                </div>
            </div>
        </div>
    );
}

export default SessionList;