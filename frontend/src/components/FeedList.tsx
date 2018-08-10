import * as React from 'react';
import { Feed } from '../Connection';
import './FeedList.css';

export type FeedListPropState = {
  feeds: Feed[],
  currentFeed: number|null,
}

export type FeedListPropDispatch = {
  clickFeed: (index: number|null) => void,
}

export type FeedListProps = FeedListPropState & FeedListPropDispatch;

class FeedList extends React.Component<FeedListProps> {
  render() {
    return (
      <div>
        <button
          className="FeedList-feed"
          onClick={() => this.props.clickFeed(null)}
          disabled={null == this.props.currentFeed}
        >
          Latest
        </button>
        {this.props.feeds.map((feed: Feed, index: number) =>
          <button
            key={index}
            className="FeedList-feed"
            onClick={() => this.props.clickFeed(feed.id)}
            disabled={feed.id == this.props.currentFeed}
          >
            {feed.title}
          </button>
        )}
      </div>
    );
  }
}

export default FeedList;
