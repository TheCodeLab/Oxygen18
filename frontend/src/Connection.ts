export type GetLatestRequest = {
  type: 'GetLatest',
  num_entries: number,
  offset: number|undefined,
};
export type GetFeedsRequest = {
  type: 'GetFeedList',
};
export type AddFeedRequest = {
  type: 'AddFeed',
  url: string,
};
export type MarkReadRequest = {
  type: 'MarkRead',
  entry_id: number,
}
export type RequestBody =
GetLatestRequest |
GetFeedsRequest |
AddFeedRequest |
MarkReadRequest;

export type RequestMessage = {
  id: number,
  body: RequestBody,
}

export type FeedEntry = {
  row_id: number,
  feed_id: number,
  title: string,
  id: string,
  updated: number,
  summary: string,
  content: string,
  is_read: boolean,
};

export type Feed = {
  id: number,
  last_update: number,
  title: string,
  url: string,
};

export type SuccessResponse = {
  type: 'Success',
};
export type ErrorResponse = {
  type: 'Error',
  error: string,
};
export type FeedEntriesResponse = {
  type: 'FeedEntries',
  list: FeedEntry[],
};
export type FeedListResponse = {
  type: 'FeedList',
  list: Feed[],
};
export type ResponseBody =
SuccessResponse |
ErrorResponse |
FeedEntriesResponse |
FeedListResponse;

export type ResponseMessage = {
  id: number,
  body: ResponseBody,
};

export class ConnectionError extends Error {
  constructor(message: string) {
    super(message);
  }
}

type PromiseResolver<T, E = string> = {
  resolve: (value: T) => void,
  reject: (reason: E) => void,
};

export default class Connection {
  socket: WebSocket;
  nextId: number;
  pendingRequests: Map<number, PromiseResolver<ResponseBody, ConnectionError>>;
  isOpen: boolean;
  pendingOpen: PromiseResolver<void>[];
  
  constructor(url: string) {
    this.nextId = 0;
    this.socket = new WebSocket(url, 'atom-client');
    this.pendingRequests = new Map();
    this.socket.onmessage = this._onResponse.bind(this);
    this.isOpen = false;
    this.pendingOpen = [];
    this.socket.onopen = this._onOpen.bind(this);
  }
  
  _onOpen() {
    this.isOpen = true;
    for (const resolver of this.pendingOpen) {
      resolver.resolve(undefined);
    }
  }
  
  _onResponse(message: MessageEvent) {
    const responseStr: string = message.data;
    var response: ResponseMessage;
    try {
      response = JSON.parse(responseStr);
    } catch (e) {
      console.log("Parsing response failed", responseStr);
      return;
    }
    console.log(">>>>>>", response);
    const request = this.pendingRequests.get(response.id);
    if (request) {
      if (response.body.type === 'Error') {
        request.reject(new ConnectionError(response.body.error));
      }
      else {
        request.resolve(response.body);
      }
    }
    else {
      console.log("No handler for response", response);
    }
    this.pendingRequests.delete(response.id);
  }
  
  _request(body: RequestBody): Promise<ResponseBody> {
    const request: RequestMessage = {
      id: ++this.nextId,
      body,
    };
    console.log("<<<<<<", request);
    const requestStr = JSON.stringify(request);
    this.socket.send(requestStr);
    return new Promise((resolve, reject) => {
      this.pendingRequests.set(request.id, {
        resolve,
        reject,
      });
    })
  }
  
  onOpen(): Promise<void> {
    if (this.isOpen) {
      return new Promise((resolve, _reject) => resolve());
    }
    else {
      return new Promise((resolve, reject) => {
        this.pendingOpen.push({
          resolve, reject
        });
      })
    }
  }
  
  getLatest(num_entries: number, offset: number|undefined): Promise<FeedEntry[]> {
    return this._request({
      type: 'GetLatest',
      num_entries,
      offset,
    }).then((response) => {
      if (response.type == 'FeedEntries') {
        return response.list;
      }
      else {
        throw new ConnectionError(`Expected FeedEntries response, got ${response.type}`);
      }
    });
  }
  
  getFeeds(): Promise<Feed[]> {
    return this._request({
      type: 'GetFeedList',
    }).then((response) => {
      if (response.type == 'FeedList') {
        return response.list;
      }
      else {
        throw new ConnectionError(`Expected FeedList response, got ${response.type}`);
      }
    });
  }
  
  addFeed(url: string): Promise<void> {
    return this._request({
      type: 'AddFeed',
      url: url,
    }).then((_response) => {
      return;
    })
  }

  markRead(rowId: number): Promise<void> {
    return this._request({
      type: 'MarkRead',
      entry_id: rowId,
    }).then((_response) => {
      return;
    })
  }
}
