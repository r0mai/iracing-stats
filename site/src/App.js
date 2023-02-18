import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';

import _default from 'react-async';
import './App.css';
import DriverList from './DriverList.js';

import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';

let darkTheme = createTheme({
    palette: {
        mode: "dark"
    }
});

function App() {
    let paramString = window.location.search.split('?')[1];
    let queryString = new URLSearchParams(paramString);

    let urlDrivers = queryString.get('drivers');
    if (!urlDrivers) {
        return "Pass in a list of drivers <url>?drivers=Driver1;Driver2;Driver3";
    }
    let drivers = urlDrivers.split(';');
    return (
        <ThemeProvider theme={darkTheme}>
            <CssBaseline />
            <DriverList drivers={drivers}/>
        </ThemeProvider>
    );
}

export default App;