import * as React from 'react';
import { Feed } from '../Connection';
import './FeedList.css';

export type FeedListProps = {
  feeds: Feed[],
}

class FeedList extends React.Component<FeedListProps> {
  render() {
    return (
      <div>
        {this.props.feeds.map((feed: Feed, index: number) =>
          <div key={index} className="FeedList-feed">
            {feed.title}
          </div>
        )}
      </div>
    );
  }
}

export default FeedList;
