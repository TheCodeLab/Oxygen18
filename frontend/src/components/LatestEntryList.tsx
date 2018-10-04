import * as React from 'react';
import { State } from "../reducers";
import FeedEntryList, { FeedEntryListPropState, FeedEntryListPropDispatch } from "./FeedEntryList";
import { connect } from 'react-redux';
import { Dispatch } from "redux";
import setRead from "../thunks/setRead";
import Connection from "../Connection";
import ConnectionContext from "../ConnectionContext";

const mapStateToProps = (state: State): FeedEntryListPropState => {
  return {
    entries: state.feedEntries.filter((entry) =>
      state.feedFilter == null || entry.feed_id == state.feedFilter),
  }
};

const mapDispatchToProps = (dispatch: Dispatch): FeedEntryListPropDispatch => {
  return {
    setRead: (entryId: number, isRead: boolean, conn: Connection) => {
      setRead(dispatch, conn, [entryId], isRead);
    }
  }
}

const LatestEntryList = connect(mapStateToProps, mapDispatchToProps)(FeedEntryList);

function LatestEntryList2(props: {}) {
  return (
    <ConnectionContext.Consumer>
      {conn => <LatestEntryList conn={conn!} />}
    </ConnectionContext.Consumer>
  );
}

export default LatestEntryList2;
