import { AddFeedEntries } from "./addFeedEntries";
import { AddFeedList } from "./addFeedList";
import { SetFeedFilter } from "./setFeedFilter";
import { MarkRead } from "./markRead";

type Action = AddFeedEntries | AddFeedList | SetFeedFilter | MarkRead;

export default Action;
