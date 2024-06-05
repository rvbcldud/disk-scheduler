use crate::{
    Request, Scheduler, VecOwner,
};

pub struct SCAN {
    requests: Vec<Request>,
    current: u16,
}

impl SCAN {
    pub fn new(current: u16) -> Self{
        Self {
            requests: Vec::new(),
            current,
        }
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
        todo!()
    }
}