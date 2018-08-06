import * as React from 'react';
import { FeedEntry } from '../Connection';
import './FeedEntry.css';
import SafeHtmlDoc from './SafeHtmlDoc';

export type FeedEntryProps = {
  entry: FeedEntry,
}

class FeedEntryView extends React.Component<FeedEntryProps> {
  render() {
    const entry = this.props.entry;
    const date = new Date(entry.updated * 1000.0);
    const parser = new DOMParser();
    const doc = parser.parseFromString(entry.content, "text/html");
    return (
      <div className='FeedEntry'>
        <h3>{entry.title}</h3>
        <p>{entry.summary}</p>
        <div>
          <SafeHtmlDoc node={doc} />
        </div>
        <span className="FeedEntry-date">{date.toDateString() + " " + date.toLocaleTimeString()}</span>
      </div>
    );
  }
}

export default FeedEntryView;
