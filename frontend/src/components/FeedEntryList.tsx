import * as React from 'react';
import { FeedEntry } from '../Connection';
import './FeedEntryList.css';

export type FeedEntryListProps = {
  entries: FeedEntry[],
}

class FeedEntryList extends React.Component<FeedEntryListProps> {
  render() {
    return (
      <div>
        {this.props.entries.map((entry: FeedEntry, index: number) => {
          const date = new Date(entry.updated * 1000.0);
          return (
            <div key={index} className='FeedEntry'>
              <h3>{entry.title}</h3>
              <p>{entry.summary}</p>
              <p>{entry.content}</p>
              <span className="FeedEntry-date">{date.toDateString() + " " + date.toLocaleTimeString()}</span>
            </div>
          );
        })}
      </div>
    );
  }
}

export default FeedEntryList;
