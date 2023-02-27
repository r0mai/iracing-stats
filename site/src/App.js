import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';

import _default from 'react-async';
import './App.css';
import DriverList from './DriverList.js';
import { theme } from './Theme.js';

import React from 'react';
import { ThemeProvider } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import Box from '@mui/material/Box';

import * as ReportType from './ReportType.js';
import * as Category from './LicenseCategory.js';

function App() {
    let [reportState, setReportState] = React.useState({
        type: ReportType.kSummary,
        category: Category.kRoad
    });

    let paramString = window.location.search.split('?')[1];
    let queryString = new URLSearchParams(paramString);

    let urlDrivers = queryString.get('drivers');
    if (!urlDrivers) {
        return "Pass in a list of drivers <url>?drivers=Driver1;Driver2;Driver3";
    }
    let drivers = urlDrivers.split(';');
    
    return (
        <ThemeProvider theme={theme}>
            <CssBaseline />
            <Box sx={{ m: 1 }}>
                <DriverList drivers={drivers} reportState={reportState} setReportState={setReportState}/>
            </Box>
        </ThemeProvider>
    );
}

export default App;