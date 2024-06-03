use std::fmt::Error;
pub trait Scheduler {
    fn next_request() -> Result<(), Error>;
}