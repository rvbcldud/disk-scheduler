use crate::{
    Request, Scheduler, VecOwner, Direction,
};


pub struct FCFS {
    requests: Vec<Request>,
    current: u16,
    _max_cylinder: u16,
    direction: Direction,
    movements: u64,
    reversals: u16,
}

impl FCFS {
    pub fn new(current: u16, max_cylinder: u16) -> Self{
        Self {
            requests: Vec::new(),
            current,
            _max_cylinder: max_cylinder,
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
    pub fn simulate_scheduling(&mut self) {
        print!("FCFS ");
        while self.length() != 0 {
            let next: Request = match self.next_request() {
                Some(next) => next,
                None => panic!("done"),
            };
            print!("{} ", next.location);
        }
        println!();
    }
}

impl VecOwner for FCFS {
    fn get_vec(&mut self) -> &mut Vec<Request> {
        &mut self.requests
    }
}

impl Scheduler for FCFS {
    fn next_request(&mut self) -> Option<Request> {
        let mut min_arrival = u16::MAX;
        let mut index: Option<usize> = None;

        // Get the request with the next closest arrival
        for (i, request) in self.requests.iter().enumerate() {
            if request.arrival < min_arrival {
                min_arrival = request.arrival;
                index = Some(i);
            }
        }

        // Count the first movement done
        if self.movements == 0 {
            self.movements = 1;
        }
            
        if !index.is_none() {

            let request = &self.requests[index.unwrap()];

            // If next request is in opposite direction, change accordingly
            if self.current > request.location
                && self.direction == Direction::HIGH {
                self.reversals += 1;
                self.direction = Direction::LOW;
            } else if self.current < request.location 
                && self.direction == Direction::LOW {
                self.reversals += 1;
                self.direction = Direction::HIGH;
            }

            self.movements += u16::abs_diff(request.location, self.current) as u64;
            self.current = request.location;

            // Service the request
            Some(self.requests.remove(index.unwrap()))
        } else {
            None
        }
    }

    fn print_info(&self) {
        println!("FCFS {} {}", self.reversals, self.movements);
    }
}