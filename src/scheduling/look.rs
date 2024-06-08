use crate::{
    Request, Scheduler, VecOwner, Direction
};

pub struct LOOK {
    requests: Vec<Request>,
    current: u16,
    max_cylinder: u16,
    direction: Direction,
    movements: u16,
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
            max_cylinder,
            direction: Direction::DEFAULT,
            movements: 0,
            reversals: 0,
            circular: false,
            low_pole: 0,
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
        let mut low_pole: u16 = u16::MAX;
        let mut high_pole: u16 = 0;
        for (i, request) in self.requests.iter().enumerate() {
            if request.location < low_pole {
                low_pole = request.location;
            } else if request.location > high_pole {
                high_pole = request.location;
            }
        }

        // println!("low: {}, high: {}", low_pole, high_pole);

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
        self.update_poles();
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
                        {
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


        // for i in 0..self.length() {
        //     println!("el: {}", self.requests[i]);
        // }

        // println!("dir: {}", self.direction);
        if !index.is_none() {
            if self.movements == 0 {
                self.movements = 1;
            }

            let request = &self.requests[index.unwrap()];

            self.movements += min_dist;
            // println!("()Next: {}", request.location);
            self.current = request.location;

            let mut dir_change: bool = false;

            match self.direction {
                Direction::HIGH => {
                    if self.current == self.high_pole  && self.requests.len() > 1 {
            // println!("len: {}", self.requests.len());
                        // println!("dir change");
                        dir_change = true;
                        self.reversals += 1;
                        self.direction = Direction::LOW;
                    }
                },
                Direction::LOW => {
                    if self.current == self.low_pole && self.requests.len() > 1 {
            // println!("len: {}", self.requests.len());
                        dir_change = true;
                        // println!("dir change");
                        self.reversals += 1;
                        self.direction = Direction::HIGH;
                    }
                },
                Direction::DEFAULT => panic!("SCAN needs direction")
            };

            if self.circular && self.direction == Direction::LOW && request.location == self.high_pole && !dir_change {
                None

            // } else if self.circular && request.location != self.low_pole 
            //         && request.location != self.high_pole
            //         && self.direction == Direction::LOW 
            //         {
            //     None

            } else {
                // println!("servicing => {}", self.requests[index.unwrap()]);
                if self.length() == 1 && self.direction == Direction::LOW && self.circular {
                    // println!("bro");
                    self.reversals += 1;
                }
                Some(self.requests.remove(index.unwrap()))

            }

            // if self.circular && prev_dir == Direction::DEFAULT && request.location != self.low_pole {
            //     None
            // } else {
            // }

        } else {
            // println!("reverse!");
            match self.direction {
                Direction::HIGH => {
                    self.current = self.max_cylinder;
                    self.direction = Direction::LOW;
                },
                Direction::LOW => {
                    self.current = 0;
                    self.direction = Direction::HIGH;
                },
                Direction::DEFAULT => panic!("SCAN needs direction")
            }
            // println!("ye");
            self.reversals += 1;
            // println!("cur: {}", self.current);
            // println!("mvmnts: {}", self.movements);
            // panic!("done");
            // println!("Done with direction: {}", self.direction);
            // match self.direction {
            //     // Set direction to be low
            //     Direction::HIGH => {
            //     println!("Current: {}", self.current);
            //     println!("Next: {}", self.max_cylinder);
            //     println!("Dist: {}", u16::abs_diff(self.current, self.max_cylinder));
            //         self.direction = Direction::LOW;
            //         // self.movements += u16::abs_diff(self.current, self.max_cylinder);
            //         self.reversals += 1;
            //         // self.current -= 1;
            //         // TODO: add movements to edge
            //         // self.current = self.max_cylinder;
            //     },
            //     Direction::LOW => {
            //     println!("Current: {}", self.current);
            //     println!("Next: {}", self.max_cylinder);
            //     println!("Dist: {}", u16::abs_diff(self.current, self.max_cylinder));
            //         self.direction = Direction::HIGH;
            //         // self.movements += u16::abs_diff(self.current, 0);
            //         // self.
            //         self.reversals += 1;
            //         // self.current += 1;
            //         // self.current = 0;
            //         // TODO: add movements to edge
            //     },
            //     Direction::DEFAULT => panic!("SCAN needs direction")
            // }

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