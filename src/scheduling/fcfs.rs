use crate::{
    Request, Scheduler, VecOwner, Direction,
};

use std::fmt::Error;
use std::result::*;

pub struct FCFS {
    requests: Vec<Request>,
    current: u16,
    max_cylinder: u16,
    direction: Direction,
    movements: u16,
    reversals: u16,
}

impl FCFS {
    pub fn new(current: u16, max_cylinder: u16) -> Self{
        Self {
            requests: Vec::new(),
            current,
            max_cylinder,
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

impl VecOwner for FCFS {
    fn get_vec(&mut self) -> &mut Vec<Request> {
        &mut self.requests
    }
}

impl Scheduler for FCFS {
    fn next_request(&mut self) -> Option<Request> {
        let mut min_arrival = u16::MAX;
        let mut index: Option<usize> = None;
        for (i, request) in self.requests.iter().enumerate() {
            if request.arrival < min_arrival {
                min_arrival = request.arrival;
                index = Some(i);
            }
        }


        // let index = self.requests
        //     .iter()
        //     .enumerate()
        //     .fold(u16::MAX, |min_arr, (i, request)| {
        //         request.arrival.min(min_arr)
        //     });
            
        if !index.is_none() {
            if self.movements == 0 {
                self.movements = 1;
            }

            let request = &self.requests[index.unwrap()];

            // println!("Current: {}", self.current);
            // println!("Next: {}", request.location);
            // println!("Directoin: {}", self.direction);

            // If moved lower and going higher
            if self.current > request.location
                && self.direction == Direction::HIGH {
                self.reversals += 1;
                self.direction = Direction::LOW;
            // If moved higher and going lower
            } else if self.current < request.location 
                && self.direction == Direction::LOW {
                self.reversals += 1;
                self.direction = Direction::HIGH;
            }

            self.movements += u16::abs_diff(request.location, self.current);
            self.current = request.location;

            Some(self.requests.remove(index.unwrap()))
        } else {
            None
        }
    }

    fn print_info(&self) {
        println!("FCFS {} {}", self.reversals, self.movements);
    }
}