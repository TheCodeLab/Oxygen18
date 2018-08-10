import * as React from 'react';
import { render } from 'react-dom';
import './index.css';
import App from './components/App';
import registerServiceWorker from './registerServiceWorker';
import { createStore } from 'redux';
import reducer from './reducers';
import { Provider } from 'react-redux';
import Connection from './Connection';
import ConnectionContext from './ConnectionContext';
import getLatest from './thunks/getLatest';
import getFeeds from './thunks/getFeeds';

const enhancer = window['devToolsExtension'] ? window['devToolsExtension']()(createStore) : createStore;
const store = enhancer(
  reducer
);

const conn = new Connection('ws://localhost:2794');

conn.onOpen().then(() => {
  getLatest(store.dispatch, conn, 15, 0);
  getFeeds(store.dispatch, conn);
})

const element = (
  <Provider store={store}>
    <ConnectionContext.Provider value={conn}>
      <App />
    </ConnectionContext.Provider>
  </Provider>
);

render(element, document.getElementById('root'));
registerServiceWorker();
