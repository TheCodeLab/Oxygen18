export type GetLatestRequest = {
    type: 'GetLatest',
};
export type AddFeedRequest = {
    type: 'AddFeed',
    url: string,
};
export type RequestBody = GetLatestRequest | AddFeedRequest;

export type RequestMessage = {
    id: number,
    body: RequestBody,
}

export type FeedEntry = {
    feedId: number,
    title: string,
    id: string,
    updated: number,
    summary: string,
    content: string,
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
    entries: [FeedEntry],
};
export type ResponseBody = SuccessResponse | ErrorResponse | FeedEntriesResponse;

export type ResponseMessage = {
    id: number,
    body: ResponseBody,
};

export class ConnectionError extends Error {
    constructor(message: string) {
        super(message);
    }
}

export default class Connection {
    socket: WebSocket;
    nextId: number;
    pendingRequests: Map<number, {
        resolve: (value: ResponseBody) => void,
        reject: (reason: string) => void,
    }>;

    constructor(url: string) {
        this.socket = new WebSocket(url, 'atom-client');
        this.pendingRequests = new Map();
        this.socket.onmessage = this._onResponse.bind(this);
    }

    _onResponse(responseStr: string) {
        const response: ResponseMessage = JSON.parse(responseStr);
        const request = this.pendingRequests[response.id];
        if (request) {
            if (response.body.type === 'Error') {
                request.reject(new ConnectionError(response.body.error));
            }
            else {
                request.resolve(response.body);
            }
        }
    }

    _request(body: RequestBody): Promise<ResponseBody> {
        const request: RequestMessage = {
            id: this.nextId++,
            body,
        };
        const requestStr = JSON.stringify(request);
        this.socket.send(requestStr);
        return new Promise((resolve, reject) => {
            this.pendingRequests.set(request.id, {
                resolve,
                reject,
            });
        })
    }

    getLatest(): Promise<[FeedEntry]> {
        return this._request({
            type: 'GetLatest',
        }).then((response) => {
            if (response.type == 'FeedEntries') {
                return response.entries;
            }
            else {
                throw new Error(`Expected FeedEntries response, got ${response.type}`);
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
}
