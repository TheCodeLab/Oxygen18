import Action from '../actions';
import { FeedEntry, Feed } from '../Connection';
import { combineReducers } from 'redux';

export type State = {
  feedEntries: FeedEntry[],
  feeds: Feed[],
  feedFilter: number|null,
};

function feedEntries(state: FeedEntry[], action: Action): FeedEntry[] {
  if (action.type == 'AddFeedEntries') {
    return state.concat(action.entries);
  }
  else if (action.type == 'MarkRead') {
    return state.map((item, index) => {
      if (item.row_id !== action.rowId) {
        return item;
      }

      return {
        ...item,
        is_read: true,
      };
    })
  }
  
  return state || [];
}

function feeds(state: Feed[], action: Action): Feed[] {
  if (action.type == 'AddFeedList') {
    return action.feeds;
  }
  
  return state || [];
}

function feedFilter(state: number|null, action: Action): number|null {
  if (action.type == 'SetFeedFilter') {
    return action.feedId;
  }

  return state || null;
}

export default combineReducers<State, Action>({
  feedEntries,
  feeds,
  feedFilter,
})
