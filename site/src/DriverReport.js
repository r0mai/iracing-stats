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
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';

function preprocessDriverSessions(sessions) {
    sessions.forEach(session => {
        session['start_time'] = new Date(session['start_time']);
    });
    sessions.sort((a, b) => a['start_time'].getTime() - b['start_time'].getTime());
}

function DriverReport({driver}) {
    let driverStatsElement;
    let roadReport;
    let ovalReport;
    let dirtRoadReport;
    let dirtOvalReport;

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
}

export default DriverReport;