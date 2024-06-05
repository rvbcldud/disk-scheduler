use crate::{
    Request, Scheduler,
};

use std::fmt::Error;
use std::result::*;

pub struct FCFS {
    requests: Vec<Request>,
}

impl FCFS {
    pub fn new() -> Self{
        Self {
            requests: Vec::new(),
        }
    }
    pub fn length(&self) -> usize{
        self.requests.len()
    }
    pub fn remove(&mut self, index: usize) {
        self.requests.remove(index);
    }
    pub fn add(&mut self, req: Request) {
        self.requests.push(req);
    }
}

impl Scheduler for FCFS {
    fn next_request(&mut self) -> Option<Request> {
        // TODO: implement finding next request
        let mut min_arrival = u16::MAX;
        // let mut request: Option<&Request> = None;
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
            Some(self.requests.remove(index.unwrap()))
        } else {
            None
        }
    }
}