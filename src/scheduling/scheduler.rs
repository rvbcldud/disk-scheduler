use crate::Request;

pub trait Scheduler {
    fn next_request(&mut self) -> Option<Request>;
}