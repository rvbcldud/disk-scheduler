use crate::{
    Request, Scheduler,
};

pub struct SSTF {
    requests: Vec<Request>,
}

impl SSTF {
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

impl Scheduler for SSTF {
    fn next_request(&mut self) -> Option<Request> {
        todo!()
    }
}