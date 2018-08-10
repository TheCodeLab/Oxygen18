import Connection from "../Connection";
import { Dispatch } from 'redux';
import Action from "../actions";
import getFeeds from "./getFeeds";

async function addFeed(dispatch: Dispatch<Action>, conn: Connection, url: string) {
  await conn.addFeed(url);
  await getFeeds(dispatch, conn);
}
export default addFeed;
