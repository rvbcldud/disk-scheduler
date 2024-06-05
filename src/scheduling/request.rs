use std::fmt::{self, Display};


pub struct Request {
    // Cylinder number location
    pub location: u16,
    // Only applicable for FCFS
    pub arrival: u16,
}

impl Request {
    pub fn new(loc: u16, arr: u16) -> Self{
        Self{
            location: loc,
            arrival: arr,
        }
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.location, self.arrival)
    }
}