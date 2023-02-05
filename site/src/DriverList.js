import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import './DriverList.css'
// import 'react-tabs/style/react-tabs.css';

function DriverList() {
  let drivers = ["Bela", "Jani", "Marcsi"];
  return (
    <Tabs>
        <TabList>
            {
                drivers.map((driver) => {
                    return <Tab>{driver}</Tab>
                })
            }
        </TabList>
        {
            drivers.map((driver) => {
                return (
                    <TabPanel>
                        <h2>{driver}</h2>
                    </TabPanel>
                );
            })
        }
    </Tabs>
  );
}

export default DriverList;