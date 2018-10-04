import * as React from 'react';
import Connection, { FeedEntry } from '../Connection';
import FeedEntryView from './FeedEntry';

export type FeedEntryListPropState = {
  entries: FeedEntry[],
}

export type FeedEntryListPropDispatch = {
  setRead: (entryId: number, isRead: boolean, conn: Connection) => void,
}

export type FeedEntryListProps = FeedEntryListPropState & FeedEntryListPropDispatch & { conn: Connection };

class FeedEntryList extends React.Component<FeedEntryListProps> {
  setRead = (entryId: number, isRead: boolean) => {
    this.props.setRead(entryId, isRead, this.props.conn);
  }

  render() {
    return (
      <div>
        {this.props.entries.map((entry: FeedEntry, index: number) =>
          <FeedEntryView key={index} entry={entry} setRead={this.setRead} />
        )}
      </div>
    );
  }
}

export default FeedEntryList;
