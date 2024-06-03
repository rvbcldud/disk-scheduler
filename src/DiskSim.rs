mod scheduling;
pub use self::scheduling::{
    fcfs::FCFS,
    request::Request,
    scheduler::Scheduler,
};

fn main() {
    println!("Hello World!");

    let mut this: FCFS = FCFS::new();

    this.requests.push(Request::new());

    println!("{}", this.requests[0]);
}