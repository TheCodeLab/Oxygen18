import * as React from 'react';
import { Component } from 'react';
import './App.css';
import LatestEntryList from './LatestEntryList';

class App extends Component {
  render() {
    return (
      <div className="App">
        <header className="App-header">
          <h1 className="App-title">Atom Reader</h1>
        </header>
        <LatestEntryList />
      </div>
    );
  }
}

export default App;
