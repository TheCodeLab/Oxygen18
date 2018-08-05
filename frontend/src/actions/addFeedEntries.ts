import { FeedEntry } from "../Connection";

export type AddFeedEntries = {
    type: 'AddFeedEntries',
    entries: FeedEntry[],
};

export function addFeedEntries(entries: FeedEntry[]): AddFeedEntries {
    return {
        type: 'AddFeedEntries',
        entries,
    };
}
