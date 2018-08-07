# Oxygen18 - Atom Reader

_**Disclaimer**: This is a work in progress, not yet usable!_

Oxygen18 is a client for keeping up with Atom/RSS feeds. It's called
Oxygen18 because it implements atom, uses react, and the backend is
written in rust.

The frontend is written in TypeScript, uses React, and
communicates with the backend via WebSocket.

The backend is written in Rust, and stores its data in an SQLite
database instead of the XML file other readers usually use.

## Development Setup

```sh
cd backend/
cargo run
```

```sh
cd frontend/
npm start
```

## License

All files contained are licensed under the **Mozilla Public License,
version 2**, if applicable or unless otherwise specified.
