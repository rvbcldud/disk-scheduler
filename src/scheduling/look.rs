use crate::{
    Request, Scheduler, VecOwner, Direction
};

pub struct LOOK {
    requests: Vec<Request>,
    current: u16,
    _max_cylinder: u16,
    direction: Direction,
    movements: u64,
    reversals: u16,
    circular: bool,
    low_pole: u16,
    high_pole: u16,
}

impl LOOK {
    pub fn new(current: u16, max_cylinder: u16) -> Self{
        Self {
            requests: Vec::new(),
            current,
            _max_cylinder: max_cylinder,
            direction: Direction::DEFAULT,
            movements: 0,
            reversals: 0,
            circular: false,
            low_pole: u16::MAX,
            high_pole: 0,
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

    pub fn is_circular(mut self) -> Self {
        self.circular = true;
        self
    }

    /// Update values for poles (the farthest request on both ends)
    pub fn update_poles(&mut self) {
        let mut low_pole: u16 = self.low_pole;
        let mut high_pole: u16 = self.high_pole;
        for (_, request) in self.requests.iter().enumerate() {
            if request.location < low_pole {
                low_pole = request.location;
            } else if request.location > high_pole {
                high_pole = request.location;
            }
        }

        self.low_pole = low_pole;
        self.high_pole = high_pole;
    }

    pub fn simulate_scheduling(&mut self) {
        if self.circular {
            print!("C-");
        }
        print!("LOOK ");
        while self.length() != 0 {
            let next: Request = match self.next_request() {
                Some(next) => next,
                None => continue,
            };
            print!("{} ", next.location);
        }
        println!();
    }
}

impl VecOwner for LOOK {
    fn get_vec(&mut self) -> &mut Vec<Request> {
        &mut self.requests
    }
}

impl Scheduler for LOOK {
    fn next_request(&mut self) -> Option<Request> {
        // Update the poles before each iteration
        //  - LOOK relies heavily on knowing where the head is in reference to these two poles
        self.update_poles();

        // If head is going the wrong direction, go towards closest pole
        if self.current <= self.low_pole && self.direction == Direction::LOW {
            self.direction = Direction::HIGH;
            self.reversals += 1;
        } 
        else if self.current >= self.high_pole && self.direction == Direction::HIGH{
            self.direction = Direction::LOW;
            self.reversals += 1;
        }

        let mut min_dist = u16::MAX;
        let mut index: Option<usize> = None;

        for (i, request) in self.requests.iter().enumerate() {
            let dist = u16::abs_diff(request.location, self.current);

            // Find next closest request in the direction of movement
            match self.direction {
                Direction::HIGH => {
                    if request.location > self.current && dist <= min_dist {
                        min_dist = dist;
                        index = Some(i);
                    }
                },
                Direction::LOW => {
                    if request.location < self.current && dist <= min_dist {
                        min_dist = dist;
                        index = Some(i);
                    } 
                },
                Direction::DEFAULT => panic!("SCAN needs direction")
            }
        }

        // Set direction to be low
        if self.movements == 0 {
            self.movements = 1;
        }

        if !index.is_none() {

            let request = &self.requests[index.unwrap()];

            self.movements += min_dist as u64;
            self.current = request.location;

            // If scheduler is C-LOOK, do not service while doing from HIGH to LOW
            if self.circular && self.direction == Direction::LOW && self.current != self.low_pole {
                return None;
            }


            // If you reach pole, turn
            match self.direction {
                Direction::HIGH => {
                    if self.current == self.high_pole  && self.requests.len() > 1 {
                        self.reversals += 1;
                        self.direction = Direction::LOW;
                    }
                },
                Direction::LOW => {
                    if self.current == self.low_pole && self.requests.len() > 1 {
                        self.reversals += 1;
                        self.direction = Direction::HIGH;
                    }
                },
                Direction::DEFAULT => panic!("SCAN needs direction")
            };

            // If the C-LOOK is servicing the last request going LOW, turn around and service it immediately
            if self.length() == 1 && self.direction == Direction::LOW && self.circular {
                self.reversals += 1;
            }

            // Service the request
            Some(self.requests.remove(index.unwrap()))
        } else {
            // Should never get here
            None
        }

    }

    fn print_info(&self) {
        if self.circular {
            print!("C-");
        }
        println!("LOOK {} {}", self.reversals, self.movements);
    }
}