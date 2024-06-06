use std::fmt::{self, Display};

use crate::Request;

#[derive(PartialEq)]
pub enum Direction {
    HIGH,
    LOW,
    DEFAULT,
}

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::LOW => write!(f, "LOW"),
            Direction::HIGH => write!(f, "HIGH"),
            Direction::DEFAULT => write!(f, "DEFAULT"),
        }
    }
}

pub trait Scheduler {
    fn next_request(&mut self) -> Option<Request>;
    fn print_info(&self);
}