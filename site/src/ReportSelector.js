import { MenuItem, FormControl, Select, IconButton } from '@mui/material';
import * as ReportType from './ReportType.js'
import * as Category from './LicenseCategory.js';
import NavigateNextIcon from '@mui/icons-material/NavigateNext';
import NavigateBeforeIcon from '@mui/icons-material/NavigateBefore';
import Stack from '@mui/system/Stack';

function ReportSelector({reportState, setReportState}) {
    let typeSelector = (
        <Select
            labelId="report-selector-label"
            id="report-selector-id"
            value={reportState.type}
            label="Report"
            onChange={(event) => setReportState({...reportState, type: event.target.value})}
            sx={{ width: 200 }}
        >
            <MenuItem value={ReportType.kSummary}>Summary</MenuItem>
            <MenuItem value={ReportType.kSessionList}>Session List</MenuItem>
            <MenuItem value={ReportType.kIRacingHistory}>IR History</MenuItem>
            <MenuItem value={ReportType.kCPIHistory}>CPI History</MenuItem>
            <MenuItem value={ReportType.kTrackUsage}>Track Usage</MenuItem>
            <MenuItem value={ReportType.kCarUsage}>Car Usage</MenuItem>
            {/* <MenuItem value={ReportType.kHistoryChart}>History Chart</MenuItem> */}
        </Select>
    );

    let categorySelector = (
        <Select
            labelId="category-selector-label"
            id="category-selector-id"
            value={reportState.category}
            label="Category"
            onChange={(event) => setReportState({...reportState, category: event.target.value})}
            sx={{ width: 200 }}
        >
            <MenuItem value={Category.kRoad}>Road</MenuItem>
            <MenuItem value={Category.kOval}>Oval</MenuItem>
            <MenuItem value={Category.kDirtRoad}>Dirt Road</MenuItem>
            <MenuItem value={Category.kDirtOval}>Dirt Oval</MenuItem>
        </Select>
    );

    let onPrevClick = () => {
        let newType = reportState.type;
        newType -= 1;
        if (newType < 0) {
            newType = ReportType.kReportTypeCount - 1;
        }
        setReportState({...reportState, type: newType});
    };
    let onNextClick = () => {
        let newType = reportState.type;
        newType += 1;
        if (newType >= ReportType.kReportTypeCount) {
            newType = 0;
        }
        setReportState({...reportState, type: newType});
    };

    return (
        <FormControl>
            <Stack direction="row" spacing={2}>
                <IconButton onClick={onPrevClick}>
                    <NavigateBeforeIcon/>
                </IconButton>
                {typeSelector}
                <IconButton onClick={onNextClick}>
                    <NavigateNextIcon/>
                </IconButton>
                {(reportState.type == ReportType.kIRacingHistory || reportState.type == ReportType.kCPIHistory) && categorySelector}
            </Stack>
        </FormControl>
    );
}

export default ReportSelector;