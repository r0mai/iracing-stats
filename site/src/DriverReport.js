import { useFetch } from "react-async";
import DriverStats from './DriverStats.js';
import CategoryReport from './CategoryReport.js';
import CarUsage from './CarUsage.js';
import { driverToQueryParam } from './Utility.js';
import {
    Category_Road,
    Category_Oval,
    Category_DirtRoad,
    Category_DirtOval
} from './LicenseCategory.js';
import TabPanel from "./TabPanel";
import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import Box from '@mui/material/Box';
import * as React from 'react';

function preprocessDriverSessions(sessions) {
    sessions.forEach(session => {
        session['start_time'] = new Date(session['start_time']);
    });
    sessions.sort((a, b) => a['start_time'].getTime() - b['start_time'].getTime());
}

function DriverReport({driver, trackCarData}) {
    let driverStatsElement;
    let roadReport;
    let ovalReport;
    let dirtRoadReport;
    let dirtOvalReport;

    let carUsage;


    let driverQueryParam = driverToQueryParam(driver);

    let headers = { Accept: "application/json" }
    {
        let { data, error, isPending, run } = useFetch("/api/v1/driver-stats?" + driverQueryParam, {headers});

        let driverStats = data;
        if (isPending) {
            driverStatsElement = "...";
        } else if (error) {
            driverStatsElement = `Something went wront: ${error.message}`;
        } else if (driverStats) {
            driverStatsElement = <DriverStats driverStats={driverStats}/>;
        }
    }

    {
        let { data, error, isPending, run } = useFetch("/api/v1/driver-sessions?" + driverQueryParam, {headers});

        let driverSessions = data;
        if (isPending) {
        } else if (error) {
        } else if (driverSessions) {
            preprocessDriverSessions(driverSessions);
            roadReport = <CategoryReport driverSessions={driverSessions} category={Category_Road}/>;
            ovalReport = <CategoryReport driverSessions={driverSessions} category={Category_Oval}/>;
            dirtRoadReport = <CategoryReport driverSessions={driverSessions} category={Category_DirtRoad}/>;
            dirtOvalReport = <CategoryReport driverSessions={driverSessions} category={Category_DirtOval}/>;
            if (trackCarData) {
                carUsage = <CarUsage driverSessions={driverSessions} trackCarData={trackCarData}/>
            }
        }
    }

    const [tabIndex, setTabIndex] = React.useState(0);
    let updateTabIndex = (event, newIndex) => setTabIndex(newIndex);

    return (
        <Box sx={{ width: "100%" }}>
            {driverStatsElement}
            {carUsage}
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
        </Box>

    );
}

export default DriverReport;