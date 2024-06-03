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