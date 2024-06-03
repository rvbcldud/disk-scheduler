use std::Result;
pub trait Scheduler {
    fn next_request() -> Return<(), Error>;
}