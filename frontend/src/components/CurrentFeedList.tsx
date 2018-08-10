import * as React from 'react';
import { State } from "../reducers";
import FeedList, { FeedListPropState, FeedListPropDispatch } from "./FeedList";
import { connect } from 'react-redux';
import { Dispatch } from "../../node_modules/redux";
import Action from "../actions";
import { setFeedFilter } from "../actions/setFeedFilter";
import addFeed from "../thunks/addFeed";
import ConnectionContext from "../ConnectionContext";
import Connection from '../Connection';

const mapStateToProps = (state: State): FeedListPropState => {
  return {
    feeds: state.feeds,
    currentFeed: state.feedFilter,
  }
};

const mapDispatchToProps = (dispatch: Dispatch<Action>): FeedListPropDispatch => {
  return {
    clickFeed: (feedId: number) => dispatch(setFeedFilter(feedId)),
    addFeed: (url: string, conn: Connection) => addFeed(dispatch, conn, url),
  }
};

const CurrentFeedList = connect(mapStateToProps, mapDispatchToProps)(FeedList);

function CurrentFeedList2(props: {}) {
  return (
    <ConnectionContext.Consumer>
      {conn => <CurrentFeedList conn={conn!} />}
    </ConnectionContext.Consumer>
  );
}

export default CurrentFeedList2;
