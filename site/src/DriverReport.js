import { useFetch } from "react-async";
import DriverStats from './DriverStats.js';
import CategoryReport from './CategoryReport.js';
import CarUsage from './CarUsage.js';
import TrackUsage from './TrackUsage.js';
import IRatingHistory from './IRatingHistory.js'
import IncidentHistory from './IncidentHistory.js'
import SessionList from "./SessionList.js";
import { driverToQueryParam } from './Utility.js';
import * as Category from './LicenseCategory.js';
import * as ReportType from './ReportType.js'
import TabPanel from "./TabPanel";
import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import Box from '@mui/material/Box';
import Grid from '@mui/material/Grid';
import Accordion from "@mui/material/Accordion";
import AccordionSummary from "@mui/material/AccordionSummary";
import AccordionDetails from "@mui/material/AccordionDetails";
import Typography from "@mui/material/Typography";
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import * as React from 'react';

function preprocessDriverSessions(sessions) {
    sessions.forEach(session => {
        session['start_time'] = new Date(session['start_time']);
    });
    sessions.sort((a, b) => a['start_time'].getTime() - b['start_time'].getTime());
}

function DriverReport({driver, driverName, trackMap, carMap, reportState}) {
    let driverInfo;
    {
        let headers = { Accept: "application/json" }
        let { data, error, isPending, run } = useFetch("/api/v1/driver-info?" + driverToQueryParam(driver), {headers});
        driverInfo = data;
    }

    if (!driverInfo || !trackMap || !carMap) {
        return;
    }

    let driverSessions = driverInfo["sessions"];
    preprocessDriverSessions(driverSessions);

    let report;
    switch (reportState.type) {
        case ReportType.kSummary:
            report = <DriverStats driverSessions={driverSessions} trackMap={trackMap} driverName={driverName}/>;
            break;
        case ReportType.kSessionList:
            report = <SessionList driverSessions={driverSessions} trackMap={trackMap} carMap={carMap}/>;
            break;
        case ReportType.kCarUsage:
            report = <CarUsage driverSessions={driverSessions} trackMap={trackMap} carMap={carMap}/>;
            break;
        case ReportType.kTrackUsage:
            report = <TrackUsage driverSessions={driverSessions} trackMap={trackMap}/>;
            break;
        case ReportType.kIRacingHistory:
            report = <IRatingHistory driverSessions={driverSessions} category={reportState.category}/>;
            break;
        case ReportType.kCPIHistory:
            report = <IncidentHistory driverSessions={driverSessions} category={reportState.category}/>;
            break;
    }

    return (
        <Grid container>
            <Grid item xs={12}>
                {report}
            </Grid>
        </Grid>
    );
}

export default DriverReport;