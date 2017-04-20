
import React from 'react'
import ReactDOM from 'react-dom'
import {Provider} from 'react-redux'
import {ConnectedRouter} from 'react-router-redux'
import store, {history} from './store'
import injectTapEventPlugin from 'react-tap-event-plugin';
import AppRoute from './AppRoute'
import './index.css'

injectTapEventPlugin();

ReactDOM.render(
    <Provider store={store}>
        <ConnectedRouter history={history}>
            <AppRoute />
        </ConnectedRouter>
    </Provider>,
    document.getElementById('root'));