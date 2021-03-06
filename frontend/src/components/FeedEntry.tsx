import * as React from 'react';
import { FeedEntry } from '../Connection';
import './FeedEntry.css';
import SafeHtmlDoc from './SafeHtmlDoc';

export type FeedEntryProps = {
  entry: FeedEntry,
  setRead: (entryId: number, read: boolean) => void,
}

class FeedEntryView extends React.Component<FeedEntryProps> {
  markRead = () => {
    this.props.setRead(this.props.entry.row_id, !this.props.entry.is_read);
  }

  render() {
    const entry = this.props.entry;
    const isRead = this.props.entry.is_read;
    const date = new Date(entry.updated * 1000.0);
    const parser = new DOMParser();
    const doc = parser.parseFromString(entry.content, "text/html");
    return (
      <div className={'FeedEntry ' + (isRead ? "FeedEntry-read" : "")} onClick={this.markRead}>
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
