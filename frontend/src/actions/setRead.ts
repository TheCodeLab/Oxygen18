export type SetRead = {
  type: 'SetRead',
  entryIds: number[],
  isRead: boolean,
};

export function setRead(entryIds: number[], isRead: boolean): SetRead {
  return {
    type: 'SetRead',
    entryIds, isRead,
  };
}
