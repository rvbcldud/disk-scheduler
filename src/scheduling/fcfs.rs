use crate::{
    Request, Scheduler,
    //Scheduler,
};

use std::fmt::Error;
use std::result::*;

pub struct FCFS {
    pub requests: Vec<Request>,
}

impl FCFS {
    pub fn new() -> Self{
        Self {
            requests: Vec::new(),
        }
    }
}

impl Scheduler for FCFS {
    fn next_request() -> Result<(), Error> {
        // TODO: implement finding next request
        Ok(())
    }
}