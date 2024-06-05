use crate::{
    Request, Scheduler, VecOwner, Direction,
};

pub struct SSTF {
    requests: Vec<Request>,
    current: u16,
    direction: Direction,
    movements: u16,
    reversals: u16,
}

impl SSTF {
    pub fn new(current: u16) -> Self{
        Self {
            requests: Vec::new(),
            current,
            direction: Direction::DEFAULT,
            movements: 0,
            reversals: 0,
        }
    }
    pub fn with_direction(mut self, direction: &str) -> Self {
        let dir = match direction {
            "H" => Direction::HIGH,
            "L" => Direction::LOW,
            _ => panic!("Invalid direction!")
        };

        self.direction = dir;
        self
    }
}

impl VecOwner for SSTF {
    fn get_vec(&mut self) -> &mut Vec<Request> {
        &mut self.requests
    }
}

impl Scheduler for SSTF {
    fn next_request(&mut self) -> Option<Request> {
        let mut min_dist = u16::MAX;
        let mut index: Option<usize> = None;
        for (i, request) in self.requests.iter().enumerate() {
            let dist = u16::abs_diff(request.location, self.current);
            if dist < min_dist {
                min_dist = dist;
                index = Some(i);
            }
        }

        if !index.is_none() {
            Some(self.requests.remove(index.unwrap()))
        } else {
            None
        }
    }

    fn print_info(&self) {
        println!("SSTF {} {}", self.reversals, self.movements);
    }
}