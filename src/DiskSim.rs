mod scheduling;
pub use self::scheduling::{
    fcfs::FCFS,
    request::Request,
    scheduler::Scheduler,
};

fn main() {
    println!("Hello World!");

    let mut this: FCFS = FCFS::new();

    this.add(Request::new(7,5));
    this.add(Request::new(5,1));
    this.add(Request::new(2,2));

    // let mut next: Request;
    // TODO: Change to enumerate
    for _ in 0..this.length() {
        let next: Request = match this.next_request() {
            Some(next) => next,
            None => panic!("bro!"),
        };
        println!("{}", next);
    }
}