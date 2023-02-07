import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import { useFetch } from "react-async";
import { isDriverCustomerID, extractCustomerID } from './Utility';
import DriverReport from './DriverReport';
import './DriverList.css'

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
                        if (custName["cust_id"] == view.custID) {
                            view.displayName = custName["name"];
                            break;
                        }
                    }
                }
            }
        }
    }

    return (
        <Tabs>
            <TabList>
                {
                    driverViews.map((view) => {
                        return (
                            <Tab key={view.driver}>
                                {view.displayName}
                            </Tab>
                        );
                    })
                }
            </TabList>
            {
                driverViews.map((view) => {
                    return (
                        <TabPanel key={view.driver} forceRender={true}>
                            <DriverReport driver={view.driver}/>
                        </TabPanel>
                    );
                })
            }
        </Tabs>
    );
}

export default DriverList;