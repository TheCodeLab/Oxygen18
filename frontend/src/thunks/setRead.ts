import Connection from "../Connection";
import { Dispatch } from 'redux';
import Action from "../actions";
import { setRead as _setRead } from "../actions/setRead";

async function setRead(dispatch: Dispatch<Action>, conn: Connection, entryIds: number[], isRead: boolean) {
  await conn.setRead(entryIds, isRead);
  dispatch(_setRead(entryIds, isRead));
}
export default setRead;
