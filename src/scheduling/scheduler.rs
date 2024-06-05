use crate::Request;

#[derive(PartialEq)]
pub enum Direction {
    HIGH,
    LOW,
    DEFAULT,
}

pub trait Scheduler {
    fn next_request(&mut self) -> Option<Request>;
    fn print_info(&self);
}