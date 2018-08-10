import * as React from 'react';
import Connection, { Feed } from '../Connection';
import './FeedList.css';

export type FeedListPropState = {
  feeds: Feed[],
  currentFeed: number|null,
}

export type FeedListPropDispatch = {
  clickFeed: (index: number|null) => void,
  addFeed: (url: string, conn: Connection) => void,
}

export type FeedListProps = FeedListPropState & FeedListPropDispatch & { conn: Connection };

class FeedList extends React.Component<FeedListProps> {
  inputRef: React.RefObject<HTMLInputElement>;

  constructor(props: FeedListProps) {
    super(props);

    this.inputRef = React.createRef();
  }

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
        <form
          action='#'
          onSubmit={() => {
            const element = this.inputRef.current!;
            this.props.addFeed(element.value, this.props.conn);
          }}
        >
          <input
            type='text' 
            placeholder='Add feed...'
            className="FeedList-feed FeedList-addNew"
            ref={this.inputRef}
          />
        </form>
      </div>
    );
  }
}

export default FeedList;
