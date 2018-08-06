import { State } from "../reducers";
import FeedList, { FeedListProps } from "./FeedList";
import { connect } from 'react-redux';

const mapStateToProps = (state: State): FeedListProps => {
  return {
    feeds: state.feeds,
  }
};

const CurrentFeedList = connect(mapStateToProps)(FeedList);

export default CurrentFeedList;
