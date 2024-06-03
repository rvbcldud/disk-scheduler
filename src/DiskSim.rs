
mod scheduling;
pub use self::scheduling::{
    fcfs::FCFS,
    request::Request,
    scheduler::Scheduler,
}

fn main() {
    println!("Hello World!");

    let this: FCFS = FCFS::new();

    this.requests.append(Request::new());
}