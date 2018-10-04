import { AddFeedEntries } from "./addFeedEntries";
import { AddFeedList } from "./addFeedList";
import { SetFeedFilter } from "./setFeedFilter";
import { SetRead } from "./setRead";

type Action = AddFeedEntries | AddFeedList | SetFeedFilter | SetRead;

export default Action;
