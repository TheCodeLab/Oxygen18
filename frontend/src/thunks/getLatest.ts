import Connection from "../Connection";
import { addFeedEntries } from "../actions/addFeedEntries";
import { Dispatch } from 'redux';
import Action from "../actions";

async function getLatest(dispatch: Dispatch<Action>, conn: Connection, num_entries: number, offset: number) {
  const feedEntries = await conn.getLatest(num_entries, offset);
  dispatch(addFeedEntries(feedEntries));
}
export default getLatest;
