import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';

import _default from 'react-async';
import './App.css';
import DriverList from './DriverList.js';
import { theme } from './Theme.js';
import { useObjectSearchParams } from './hooks/useObjectSearchParams.js';

import React from 'react';
import { ThemeProvider } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import Box from '@mui/material/Box';

import * as ReportType from './ReportType.js';
import * as Category from './LicenseCategory.js';

function App() {
    let [state, setState] = useObjectSearchParams();

    return (
        <ThemeProvider theme={theme}>
            <CssBaseline />
            <Box sx={{ m: 1 }}>
                <DriverList state={state} setState={setState}/>
            </Box>
        </ThemeProvider>
    );
}

export default App;