import { State } from "../reducers";
import FeedEntryList, { FeedEntryListProps } from "./FeedEntryList";
import { connect } from 'react-redux';

const mapStateToProps = (state: State): FeedEntryListProps => {
    return {
        entries: state.feedEntries,
    }
};

const LatestEntryList = connect(mapStateToProps)(FeedEntryList);

export default LatestEntryList;
