import { MenuItem, FormControl, Select } from '@mui/material';
import * as ReportType from './ReportType.js'

function ReportSelector({reportState, setReportState}) {
    return (
        <FormControl>
            <Select
                labelId="report-selector-label"
                id="report-selector-id"
                value={reportState.type}
                label="Report"
                onChange={(event) => setReportState({...reportState, type: event.target.value})}
            >
                <MenuItem value={ReportType.kSummary}>Summary</MenuItem>
                <MenuItem value={ReportType.kIRacingHistory}>IR History</MenuItem>
                <MenuItem value={ReportType.kCPIHistory}>CPI History</MenuItem>
                <MenuItem value={ReportType.kTrackUsage}>Track Usage</MenuItem>
                <MenuItem value={ReportType.kCarUsage}>Car Usage</MenuItem>
            </Select>
        </FormControl>
    );
}

export default ReportSelector;