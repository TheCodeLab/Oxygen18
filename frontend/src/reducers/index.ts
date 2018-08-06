import Action from '../actions';
import { FeedEntry, Feed } from '../Connection';
import { combineReducers } from 'redux';

export type State = {
    feedEntries: FeedEntry[],
    feeds: Feed[],
};

function feedEntries(state: FeedEntry[], action: Action): FeedEntry[] {
    if (action.type == 'AddFeedEntries') {
        return state.concat(action.entries);
    }

    return state || [];
}

function feeds(state: Feed[], action: Action): Feed[] {
    if (action.type == 'AddFeedList') {
        return action.feeds;
    }

    return state || [];
}

export default combineReducers<State, Action>({
    feedEntries,
    feeds
})
