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
  else if (action.type == 'SetRead') {
    return state.map((item, _index) => {
      // Array.includes is not available in ES5
      if (action.entryIds.indexOf(item.row_id) <= -1) {
        return item;
      }

      return {
        ...item,
        is_read: action.isRead,
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
