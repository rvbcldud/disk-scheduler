use crate::{
    Request,
    //Scheduler,
};

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