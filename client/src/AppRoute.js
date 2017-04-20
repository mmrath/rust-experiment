import React from "react";
import {BrowserRouter as Router, Redirect, Route, Switch} from "react-router-dom";
import NotFound from "./components/NotFound";
import Home from "./containers/Home";
import App from "./containers/App";

function AppRoute() {
    return (
        <Router>
            <App>
                <Switch>
                    <Redirect from="/" exact to="/home"/>
                    <Route path="/home" exact component={Home}/>
                    <Route component={NotFound}/>
                </Switch>
            </App>
        </Router>
    )
}

export default AppRoute;