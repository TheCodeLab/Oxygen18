export type SetFeedFilter = {
  type: 'SetFeedFilter',
  feedId: number|null,
};

export function setFeedFilter(feedId: number|null): SetFeedFilter {
  return {
    type: 'SetFeedFilter',
    feedId,
  };
}
