import * as React from 'react';
import Connection, { FeedEntry } from '../Connection';
import FeedEntryView from './FeedEntry';

export type FeedEntryListPropState = {
  entries: FeedEntry[],
}

export type FeedEntryListPropDispatch = {
  markRead: (entryId: number, conn: Connection) => void,
}

export type FeedEntryListProps = FeedEntryListPropState & FeedEntryListPropDispatch & { conn: Connection };

class FeedEntryList extends React.Component<FeedEntryListProps> {
  constructor(props: FeedEntryListProps) {
    super(props);
    this.markRead = this.markRead.bind(this);
  }

  markRead(entryId: number) {
    this.props.markRead(entryId, this.props.conn);
  }

  render() {
    return (
      <div>
        {this.props.entries.map((entry: FeedEntry, index: number) =>
          <FeedEntryView key={index} entry={entry} markRead={this.markRead} />
        )}
      </div>
    );
  }
}

export default FeedEntryList;
