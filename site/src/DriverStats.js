import './DriverStats.css'
import { toHours, round } from './Utility.js'

function DriverStats({driverStats}) {
    return (
        <table class="driver-stats-table">
            <tbody>
                <tr>
                    <td>Name:</td>
                    <td>{driverStats["name"]}</td>
                </tr>
                <tr>
                    <td>Licenses:</td>
                    <td>...licenses...</td>
                </tr>
                <tr>
                    <td>Total laps:</td>
                    <td>{driverStats["laps"]}</td>
                </tr>
                <tr>
                    <td>Total time:</td>
                    <td>{round(toHours(driverStats["time"]), 1) + "h"}</td>
                </tr>
                <tr>
                    <td>Total distance:</td>
                    <td>{round(driverStats["distance"], 1) + "km"}</td>
                </tr>
            </tbody>
        </table>
    );
}

export default DriverStats;