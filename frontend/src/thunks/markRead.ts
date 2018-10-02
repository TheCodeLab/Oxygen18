import Connection from "../Connection";
import { Dispatch } from 'redux';
import Action from "../actions";
import { markRead as makeMarkRead } from "../actions/markRead";

async function markRead(dispatch: Dispatch<Action>, conn: Connection, id: number) {
  await conn.markRead(id);
  dispatch(makeMarkRead(id));
}
export default markRead;
