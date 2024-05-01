import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';

import _default from 'react-async';
import './App.css';
import TeamStats from './TeamStats.js';
import DriverList from './DriverList.js';
import { theme } from './Theme.js';
import { useObjectSearchParams } from './hooks/useObjectSearchParams.js';

import React from 'react';
import { ThemeProvider } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import Box from '@mui/material/Box';

function AppPage() {
    let [state, setState] = useObjectSearchParams();

    if (state["page"] === "driver-stats") {
        return <DriverList state={state} setState={setState}/>
    } else if (state["page"] === "team-stats") {
        setState({page: "team-stats", team: state["team"]});
        return <TeamStats state={state} setState={setState}/>
    } else {
        setState({...state, page: "driver-stats"});
        return <DriverList state={state} setState={setState}/>
    }
}

function App() {

    return (
        <ThemeProvider theme={theme}>
            <CssBaseline />
            <Box sx={{ m: 1 }}>
                {
                    AppPage()
                }
            </Box>
        </ThemeProvider>
    );
}

export default App;