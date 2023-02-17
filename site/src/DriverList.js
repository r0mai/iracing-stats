import { useFetch } from "react-async";
import { isDriverCustomerID, extractCustomerID } from './Utility';
import DriverReport from './DriverReport';
import TabPanel from "./TabPanel";

import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import Box from '@mui/material/Box';
import * as React from 'react';

function DriverList({drivers}) {

    let driverViews = [];
    let custIDs = [];
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

    // conditional useFetch is not allowed
    /*if (custIDs.length != 0)*/ {
        let headers = { Accept: "application/json" }
        let { data, error, isPending, run } = useFetch("/api/v1/customer-names?cust_ids=" + custIDs.join(';'), {headers});

        let custNames = data;
        if (isPending) {
        } else if (error) {
        } else if (custNames) {
            for (let view of driverViews) {
                if (view.custID) {
                    for (let custName of custNames) {
                        if (custName["cust_id"] === view.custID) {
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
        <Box sx={{ flexGrow: 1, bgcolor: 'background.paper', display: 'flex', height: '100%' }}>
            <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
                <Tabs value={tabIndex} onChange={updateTabIndex} orientation="vertical" variant="scrollable">
                    {
                        driverViews.map((view) => 
                            <Tab label={view.displayName} key={view.driver} />
                        )
                    }
                </Tabs>
            </Box>
            {
                driverViews.map((view, i) => {
                    return (
                        <TabPanel value={tabIndex} index={i}>
                            <DriverReport driver={view.driver}/>
                        </TabPanel>
                    );
                })
            }
        </Box>
    );
}

export default DriverList;