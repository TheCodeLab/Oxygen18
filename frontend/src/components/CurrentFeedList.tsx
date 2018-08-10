import { State } from "../reducers";
import FeedList, { FeedListPropState, FeedListPropDispatch } from "./FeedList";
import { connect } from 'react-redux';
import { Dispatch } from "../../node_modules/redux";
import Action from "../actions";
import { setFeedFilter } from "../actions/setFeedFilter";

const mapStateToProps = (state: State): FeedListPropState => {
  return {
    feeds: state.feeds,
    currentFeed: state.feedFilter,
  }
};

const mapDispatchToProps = (dispatch: Dispatch<Action>): FeedListPropDispatch => {
  return {
    clickFeed: (feedId: number) => dispatch(setFeedFilter(feedId)),
  }
};

const CurrentFeedList = connect(mapStateToProps, mapDispatchToProps)(FeedList);

export default CurrentFeedList;
