use crate::{
    Request, Scheduler, VecOwner, Direction
};

pub struct SCAN {
    requests: Vec<Request>,
    current: u16,
    max_cylinder: u16,
    direction: Direction,
    movements: u16,
    reversals: u16,
    circular: bool,
}

impl SCAN {
    pub fn new(current: u16, max_cylinder: u16) -> Self{
        Self {
            requests: Vec::new(),
            current,
            max_cylinder,
            direction: Direction::DEFAULT,
            movements: 0,
            reversals: 0,
            circular: false,
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

    pub fn simulate_scheduling(&mut self) {
        if self.circular {
            print!("C-");
        }
        print!("SCAN ");
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

impl VecOwner for SCAN {
    fn get_vec(&mut self) -> &mut Vec<Request> {
        &mut self.requests
    }
}

impl Scheduler for SCAN {
    fn next_request(&mut self) -> Option<Request> {
        let mut min_dist = u16::MAX;
        let mut index: Option<usize> = None;
        // Find next closest request in the direction of movement
        for (i, request) in self.requests.iter().enumerate() {
            let dist = u16::abs_diff(request.location, self.current);

            match self.direction {
                Direction::HIGH => {
                    if request.location > self.current && dist < min_dist {
                        min_dist = dist;
                        index = Some(i);
                    }
                },
                Direction::LOW => {
                    if request.location < self.current 
                            && dist < min_dist
                            && !self.circular {
                        min_dist = dist;
                        index = Some(i);
                    } 
                },
                Direction::DEFAULT => panic!("SCAN needs direction")
            }
            // if dist < min_dist {
            //     min_dist = dist;
            //     index = Some(i);
            // }
        }


            if self.movements == 0 {
                self.movements = 1;
            }

        if !index.is_none() {

            let request = &self.requests[index.unwrap()];
            self.movements += min_dist;
            // println!("()Next: {}", request.location);
            self.current = request.location;

            Some(self.requests.remove(index.unwrap()))
        } else {
            // println!("Done with direction: {}", self.direction);
            match self.direction {
                // Set direction to be low
                Direction::HIGH => {
                // println!("Current: {}", self.current);
                // println!("Next: {}", self.max_cylinder);
                // println!("Dist: {}", u16::abs_diff(self.current, self.max_cylinder));
                    self.direction = Direction::LOW;
                    self.movements += u16::abs_diff(self.current, self.max_cylinder);
                    self.reversals += 1;
                    // TODO: add movements to edge
                    self.current = self.max_cylinder;
                },
                Direction::LOW => {
                    self.direction = Direction::HIGH;
                    self.movements += u16::abs_diff(self.current, 0);
                    self.reversals += 1;
                    self.current = 0;
                    // TODO: add movements to edge
                },
                Direction::DEFAULT => panic!("SCAN needs direction")
            }

            None
        }
    }

    fn print_info(&self) {
        if self.circular {
            print!("C-");
        }
        println!("SCAN {} {}", self.reversals, self.movements);
    }
}