import Action from '../actions';
import { FeedEntry } from '../Connection';
import { combineReducers } from 'redux';

export type State = {
    feedEntries: FeedEntry[],
};

function feedEntries(state: FeedEntry[], action: Action): FeedEntry[] {
    if (action.type == 'AddFeedEntries') {
        return state.concat(action.entries);
    }

    return state || [];
}

export default combineReducers<State, Action>({
    feedEntries,
})
