import { useFetch } from "react-async";
import { isDriverCustomerID, extractCustomerID, mapifyTrackData, mapifyCarData } from './Utility';
import DriverReport from './DriverReport';
import ReportSelector from './ReportSelector.js';
import TabPanel from "./TabPanel";

import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import Grid from '@mui/material/Grid';
import * as React from 'react';

function DriverList({state, setState}) {

    let driverViews = [];
    let custIDs = [];
    let drivers = state["drivers"].split(';');
    for (let driver of drivers) {
        let view = {
            driver: driver,
            displayName: driver,
            custID: null
        };
        if (isDriverCustomerID(driver)) {
            let custID = extractCustomerID(driver);
            view.displayName = "...";
            view.custID = custID;
            custIDs.push(custID);
        }
        driverViews.push(view);
    }

    let headers = { Accept: "application/json" }

    let trackMap;
    let carMap;
    {
        let { data, error, isPending, run } = useFetch("/api/v1/track-car-data", {headers});
        if (data) {
            trackMap = mapifyTrackData(data["tracks"]);
            carMap = mapifyCarData(data["cars"]);
        }
    }

    // conditional useFetch is not allowed
    /*if (custIDs.length != 0)*/ {
        let { data, error, isPending, run } = useFetch("/api/v1/customer-names?cust_ids=" + custIDs.join(';'), {headers});

        let custNames = data;
        if (custNames) {
            for (let view of driverViews) {
                if (view.custID) {
                    for (let custName of custNames) {
                        if (custName["cust_id"].toString() === view.custID) {
                            view.displayName = custName["name"];
                            break;
                        }
                    }
                }
            }
        }
    }

    const [tabIndex, setTabIndex] = React.useState(0);
    let updateTabIndex = (event, newIndex) => setTabIndex(newIndex);

    return (
            <Grid container sx={{ flexGrow: 1, display: 'flex', height: '100%' }}>
                <Grid item xs={2} sx={{ borderBottom: 1, borderColor: 'divider' }}>
                    <Tabs value={tabIndex} onChange={updateTabIndex} orientation="vertical" variant="scrollable">
                        {
                            driverViews.map((view) => 
                                <Tab label={view.displayName} key={view.driver} />
                            )
                        }
                    </Tabs>
                </Grid>
                <Grid item xs={10}>
                {
                    driverViews.map((view, i) => {
                        return (
                            <TabPanel value={tabIndex} index={i}>
                                <ReportSelector
                                    state={state}
                                    setState={setState}
                                />
                                <DriverReport
                                    driver={view.driver}
                                    driverName={view.displayName}
                                    trackMap={trackMap}
                                    carMap={carMap}
                                    state={state}
                                />
                            </TabPanel>
                        );
                    })
                }
                </Grid>
        </Grid>
    );
}

export default DriverList;