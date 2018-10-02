export type MarkRead = {
  type: 'MarkRead',
  rowId: number,
};

export function markRead(rowId: number): MarkRead {
  return {
    type: 'MarkRead',
    rowId,
  };
}
