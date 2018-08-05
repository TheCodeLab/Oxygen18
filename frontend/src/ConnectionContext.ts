import * as React from 'react';
import Connection from './Connection';

const ConnectionContext = React.createContext<Connection|null>(null);

export default ConnectionContext;
