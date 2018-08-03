import * as React from 'react';
import { render } from 'react-dom';
import './index.css';
import App from './components/App';
import registerServiceWorker from './registerServiceWorker';
import { createStore } from 'redux';
import Action from './actions';
import { reducer, State } from './reducers';
import { Provider } from 'react-redux';

const store = createStore<State, Action, {}, {}>(reducer);

const element =
    <Provider store={store}>
        <App />
    </Provider>;

render(element, document.getElementById('root'));
registerServiceWorker();
