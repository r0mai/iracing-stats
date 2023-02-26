import { useFetch } from "react-async";
import DriverStats from './DriverStats.js';
import CategoryReport from './CategoryReport.js';
import CarUsage from './CarUsage.js';
import TrackUsage from './TrackUsage.js';
import SessionList from "./SessionList.js";
import { driverToQueryParam } from './Utility.js';
import {
    Category_Road,
    Category_Oval,
    Category_DirtRoad,
    Category_DirtOval
} from './LicenseCategory.js';
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
    let driverStatsElement;
    let roadReport;
    let ovalReport;
    let dirtRoadReport;
    let dirtOvalReport;

    let carUsage;
    let trackUsage;
    let sessionList;

    let driverQueryParam = driverToQueryParam(driver);

    {
        let headers = { Accept: "application/json" }
        let { data, error, isPending, run } = useFetch("/api/v1/driver-info?" + driverQueryParam, {headers});

        let driverInfo = data;
        if (driverInfo) {
            let driverSessions = driverInfo["sessions"];
            preprocessDriverSessions(driverSessions);
            roadReport = <CategoryReport driverSessions={driverSessions} category={Category_Road}/>;
            ovalReport = <CategoryReport driverSessions={driverSessions} category={Category_Oval}/>;
            dirtRoadReport = <CategoryReport driverSessions={driverSessions} category={Category_DirtRoad}/>;
            dirtOvalReport = <CategoryReport driverSessions={driverSessions} category={Category_DirtOval}/>;
            if (trackMap && carMap) {
                carUsage = <CarUsage driverSessions={driverSessions} trackMap={trackMap} carMap={carMap}/>
                trackUsage = <TrackUsage driverSessions={driverSessions} trackMap={trackMap}/>
                driverStatsElement = <DriverStats driverSessions={driverSessions} trackMap={trackMap} driverName={driverName}/>;
                sessionList = <SessionList driverSessions={driverSessions} trackMap={trackMap} carMap={carMap}/>
            }
        }
    }

    const [tabIndex, setTabIndex] = React.useState(0);
    let updateTabIndex = (event, newIndex) => setTabIndex(newIndex);

    let report;
    switch (reportState.type) {
        case ReportType.kSummary:
            report = driverStatsElement;
            break;
        case ReportType.kCarUsage:
            report = carUsage;
            break;
        case ReportType.kTrackUsage:
            report = trackUsage;
            break;
    }

    return (
        <Grid container>
            <Grid item xs={12}>
                {report}
            </Grid>
        </Grid>
    );

    return (
        <Grid container>
            <Grid item xs={6}>
                {driverStatsElement}
            </Grid>
            <Box width="100%"/>

            <Grid item xs={12}>
                <Accordion>
                    <AccordionSummary expandIcon={<ExpandMoreIcon />}>
                        <Typography>
                            Session list
                        </Typography>
                    </AccordionSummary>
                    <AccordionDetails>
                        {sessionList}
                    </AccordionDetails>
                </Accordion>
                <Accordion>
                    <AccordionSummary expandIcon={<ExpandMoreIcon />}>
                        <Typography>
                            Car usage stats
                        </Typography>
                    </AccordionSummary>
                    <AccordionDetails>
                        {carUsage}
                    </AccordionDetails>
                </Accordion>
                <Accordion>
                    <AccordionSummary expandIcon={<ExpandMoreIcon />}>
                        <Typography>
                            Track usage stats
                        </Typography>
                    </AccordionSummary>
                    <AccordionDetails>
                        {trackUsage}
                    </AccordionDetails>
                </Accordion>
                <Accordion>
                    <AccordionSummary expandIcon={<ExpandMoreIcon />}>
                        <Typography>
                            Category stats
                        </Typography>
                    </AccordionSummary>
                    <AccordionDetails>
                        <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
                            <Tabs value={tabIndex} onChange={updateTabIndex}>
                                <Tab label="Road" />
                                <Tab label="Oval" />
                                <Tab label="Dirt Road" />
                                <Tab label="Dirt Oval" />
                            </Tabs>
                        </Box>
                        <TabPanel value={tabIndex} index={0}>
                            {roadReport}
                        </TabPanel>
                        <TabPanel value={tabIndex} index={1}>
                            {ovalReport}
                        </TabPanel>
                        <TabPanel value={tabIndex} index={2}>
                            {dirtRoadReport}
                        </TabPanel>
                        <TabPanel value={tabIndex} index={3}>
                            {dirtOvalReport}
                        </TabPanel>
                    </AccordionDetails>
                </Accordion>
            </Grid>
        </Grid>
    );
}

export default DriverReport;