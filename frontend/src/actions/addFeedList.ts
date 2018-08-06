import { Feed } from "../Connection";

export type AddFeedList = {
    type: 'AddFeedList',
    feeds: Feed[],
};

export function addFeedList(feeds: Feed[]): AddFeedList {
    return {
        type: 'AddFeedList',
        feeds,
    };
}
