# Oxygen18 Protocol

Oxygen18 uses a WebSocket-based protocol for sending feed information to
the client.

The basic structure is by sending JSON encoded messages over the
WebSocket channel.

This document will serve as a living draft of the protocol used between
the client and server, and will evolve over time.

## Framing

### Requests

Clients can send requests to the server.

```js
{
  id: integer, // Monotonically incrementing. Must be >= 0.
  body: {
    type: string,
    ...
  }
}
```

### Responses

The server will send 1 response message for each request it receives.

```js
{
  id: integer, // The same value as `request.id`.
  body: {
    type: string,
    ...
  }
}
```

### Notifications

Not yet used. These will be used for the server to notify the client of
changes, and the client will not respond to them.

```js
{
  id: integer, // Monotonically decrementing. Must be < 0.
  body: {
    type: string,
    ...
  }
}
```

## Request Types

For all requests, `Error` is a valid response.

### GetLatest

Returns an array of `FeedEntry` objects.

```js
{
  type: 'GetLatest',
}
```

#### Valid Responses

- `FeedEntries`

### GetFeeds

Returns an array of `Feed` objects.

```js
{
  type: 'FeedEntries',
}
```

#### Valid Responses

- `FeedList`

### AddFeed

Subscribe to a new feed by URL.

```js
{
  type: 'AddFeed',
  url: string,
}
```

#### Valid Responses

- `Success`

## Response Types

### Success

Acknowledges servicing of the request.

```js
{
  type: 'Success',
}
```

### Error

Failed to service the request, possibly because the request was invalid.

```js
{
  type: 'Error',
  error: string,
}
```

### FeedEntries

A list of `FeedEntry` objects.

```js
{
  type: 'FeedEntries',
  list: FeedEntry[],
}
```

### FeedList

A list of `Feed` objects.

```js
{
  type: 'FeedList',
  list: Feed[],
}
```

## Data Types

### Feed

An object representing a subscription to a given Atom or RSS feed.

```js
{
  id: integer,
  last_update: integer, // Seconds since Unix Epoch.
  title: string,
  url: string,
}
```

### Feed Entry

An object representing a single entry from `Feed`.

```js
{
  feedId: integer,
  title: string,
  id: string,
  updated: integer, // Seconds since Unix Epoch.
  summary: string,
  content: string, // Can be HTML.
}
```
