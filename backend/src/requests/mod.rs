use rusqlite::{Connection, Error as RusqliteError};
use protocol::{Request, Response};

mod get_latest;
mod add_feed;
mod get_feeds;

#[derive(Debug)]
pub enum Error {
    Rusqlite(RusqliteError),
}

impl From<RusqliteError> for Error {
    fn from(err: RusqliteError) -> Error {
        Error::Rusqlite(err)
    }
}

pub fn process(request: Request, conn: &mut Connection) -> Result<Response, Error> {
    match request {
        Request::GetLatest(request) => get_latest::get_latest(request, conn),
        Request::AddFeed(request) => add_feed::add_feed(request, conn),
        Request::GetFeedList(request) => get_feeds::get_feeds(request, conn),
    }
}
