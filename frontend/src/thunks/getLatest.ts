import Connection from "../Connection";
import { addFeedEntries } from "../actions/addFeedEntries";
import { Dispatch } from 'redux';
import Action from "../actions";

async function getLatest(dispatch: Dispatch<Action>, conn: Connection) {
  const feedEntries = await conn.getLatest();
  dispatch(addFeedEntries(feedEntries));
}
export default getLatest;
