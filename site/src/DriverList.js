import { useFetch } from "react-async";
import { mapifyTrackData, mapifyCarData } from './Utility';
import DriverReport from './DriverReport';
import ReportSelector from './ReportSelector.js';
import TabPanel from "./TabPanel";

import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import Grid from '@mui/material/Grid';
import * as React from 'react';

function DriverList({state, setState}) {

    let driverViews = [];

    let drivers = state["drivers"];
    let team = state["team"];

    let suffix = "";
    if (drivers) {
        suffix = "?drivers=" + drivers;
    } else if (team) {
        suffix = "?team=" + team;
    }

    let headers = { Accept: "application/json" }
    let { data, error, isPending, run } = useFetch("/api/v1/customers" + suffix, {headers});

    let custNames = data;
    if (custNames) {
        for (let custName of custNames) {
            let view = {
                driver: custName["name"],
                displayName: custName["name"],
                custID: custName["cust_id"]
            };
            driverViews.push(view);
        }
    }

    let trackMap;
    let carMap;
    {
        let { data, error, isPending, run } = useFetch("/api/v1/track-car-data", {headers});
        if (data) {
            trackMap = mapifyTrackData(data["tracks"]);
            carMap = mapifyCarData(data["cars"]);
        }
    }

    let findDriverIdx = (driver) => {
        let idx = driverViews.findIndex((view) => view.driver === driver);
        return idx === -1 ? 0 : idx;
    };

    let currentIdx = findDriverIdx(state["selected"]);
    let updateTabIndex = (event, newIndex) => setState({...state, selected: driverViews[newIndex].driver}); 

    return (
            <Grid container spacing={1} sx={{ flexGrow: 1, display: 'flex', height: '100%' }}>
                <Grid item xs={2} sx={{ borderBottom: 1, borderColor: 'divider' }}>
                    <Tabs
                        value={currentIdx}
                        onChange={updateTabIndex}
                        orientation="vertical"
                        variant="scrollable"
                        sx={{ borderRight: 1, borderColor: 'divider' }}
                    >
                        {
                            driverViews.map((view) => 
                                <Tab label={view.displayName} key={view.driver} sx={{ minHeight: 'auto', padding: '6px 6px' }} />
                            )
                        }
                    </Tabs>
                </Grid>
                <Grid item xs={10}>
                {
                    driverViews.map((view, i) => {
                        return (
                            <TabPanel currentValue={currentIdx} selfValue={i}>
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