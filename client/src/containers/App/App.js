import React from "react";
import MuiThemeProvider from "material-ui/styles/MuiThemeProvider";
import ThemeDefault from "./theme-default";
import Layout from "./Layout";

function App({children}) {
    return (
        <MuiThemeProvider muiTheme={ThemeDefault}>
            <Layout>
                {children}
            </Layout>
        </MuiThemeProvider>
    );
}


export default App
