use std::fmt::{self, Display};


pub struct Request {
    // Cylinder number location
    pub location: u8,
    // Only applicable for FCFS
    pub arrival: u8,
}

impl Request {
    pub fn new() -> Self{
        Self{
            location: 0,
            arrival: 0,
        }
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.location, self.arrival)
    }
}