import Connection from "../Connection";
import { Dispatch } from 'redux';
import Action from "../actions";
import { addFeedList } from "../actions/addFeedList";

async function getFeeds(dispatch: Dispatch<Action>, conn: Connection) {
  const feeds = await conn.getFeeds();
  dispatch(addFeedList(feeds));
}
export default getFeeds;
