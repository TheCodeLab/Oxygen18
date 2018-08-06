import * as React from 'react';
import { FeedEntry } from '../Connection';
import FeedEntryView from './FeedEntry';

export type FeedEntryListProps = {
  entries: FeedEntry[],
}

class FeedEntryList extends React.Component<FeedEntryListProps> {
  render() {
    return (
      <div>
        {this.props.entries.map((entry: FeedEntry, index: number) =>
          <FeedEntryView key={index} entry={entry} />
        )}
      </div>
    );
  }
}

export default FeedEntryList;
