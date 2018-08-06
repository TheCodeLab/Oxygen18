import * as React from 'react';
import { Component } from 'react';
import './App.css';
import LatestEntryList from './LatestEntryList';
import CurrentFeedList from './CurrentFeedList';

class App extends Component {
  render() {
    return (
      <div className="App">
        <header className="App-header">
          <h1 className="App-title">Atom Reader</h1>
        </header>
        <div className="App-sidebar">
          <CurrentFeedList />
        </div>
        <div className="App-content">
          <LatestEntryList />
        </div>
      </div>
    );
  }
}

export default App;
