import { useFetch } from "react-async";
import DriverStats from './DriverStats.js'
import CategoryReport from './CategoryReport'
import { driverToQueryParam } from './Utility.js';
import {
    Category_Road,
    Category_Oval,
    Category_DirtRoad,
    Category_DirtOval
} from './LicenseCategory.js';
// import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
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

function CategoryTabPanel({children, value, index}) {
    return (
        <div
            role="tabpanel"
            hidden={value !== index}
            id={`simple-tabpanel-${index}`}
        >
            {value === index && (
                <Box sx={{ p: 3 }}>
                    {children}
                </Box>
            )}
        </div>
    );
}

function DriverReport({driver}) {
    let driverStatsElement;
    let roadReport;
    let ovalReport;
    let dirtRoadReport;
    let dirtOvalReport;

    const [tabIndex, setTabIndex] = React.useState(0);

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
        }
    }

    let updateTabIndex = (event, newIndex) => setTabIndex(newIndex);

    return (
        <Box sx={{ width: "100%" }}>
            <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
                <Tabs value={tabIndex} onChange={updateTabIndex}>
                    <Tab label="Road" />
                    <Tab label="Oval" />
                    <Tab label="Dirt Road" />
                    <Tab label="Dirt Oval" />
                </Tabs>
            </Box>
            <CategoryTabPanel value={tabIndex} index={0}>
                {roadReport}
            </CategoryTabPanel>
            <CategoryTabPanel value={tabIndex} index={1}>
                {ovalReport}
            </CategoryTabPanel>
            <CategoryTabPanel value={tabIndex} index={2}>
                {dirtRoadReport}
            </CategoryTabPanel>
            <CategoryTabPanel value={tabIndex} index={3}>
                {dirtOvalReport}
            </CategoryTabPanel>
        </Box>

    );

    /*
    return (
        <div>
            {driverStatsElement}
            <Tabs>
                <TabList>
                    <Tab>Road</Tab>
                    <Tab>Oval</Tab>
                    <Tab>Dirt Road</Tab>
                    <Tab>Dirt Oval</Tab>
                </TabList>
                <TabPanel forceRender={false}>
                    {roadReport}
                </TabPanel>
                <TabPanel forceRender={false}>
                    {ovalReport}
                </TabPanel>
                <TabPanel forceRender={false}>
                    {dirtRoadReport}
                </TabPanel>
                <TabPanel forceRender={false}>
                    {dirtOvalReport}
                </TabPanel>
            </Tabs>
        </div>
    );
    */
}

export default DriverReport;