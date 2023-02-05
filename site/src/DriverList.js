import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import './DriverList.css'
import DriverReport from './DriverReport';
// import 'react-tabs/style/react-tabs.css';

function DriverList({drivers}) {
    return (
        <Tabs>
            <TabList>
                {
                    drivers.map((driver) => {
                        return (
                            <Tab key={driver}>
                                {driver}
                            </Tab>
                        );
                    })
                }
            </TabList>
            {
                drivers.map((driver) => {
                    return (
                        <TabPanel key={driver} forceRender={true}>
                            <DriverReport driver={driver}/>
                        </TabPanel>
                    );
                })
            }
        </Tabs>
    );
}

export default DriverList;