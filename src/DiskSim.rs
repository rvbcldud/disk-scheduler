mod scheduling;
pub use self::scheduling::{
    fcfs::FCFS,
    sstf::SSTF,
    request::Request,
    scheduler::{Scheduler, Direction},
    vec_owner::VecOwner
};

fn main() {
    let mut first_come: FCFS = FCFS::new(8);

    first_come.add(Request::new(1,0));
    first_come.add(Request::new(20,1));
    first_come.add(Request::new(10,2));

    // let mut next: Request;
    // TODO: Change to enumerate
    for _ in 0..first_come.length() {
        let next: Request = match first_come.next_request() {
            Some(next) => next,
            None => panic!("bro!"),
        };
        println!("{}", next);
    }

    first_come.print_info();

    println!();

    let mut shortest: SSTF = SSTF::new(8)
        .with_direction("H");

    shortest.add(Request::new(7,0));
    shortest.add(Request::new(5,0));
    shortest.add(Request::new(8,0));
    shortest.add(Request::new(1,0));

    for _ in 0..shortest.length() {
        let next: Request = match shortest.next_request() {
            Some(next) => next,
            None => panic!("bro!"),
        };
        println!("{}", next);
    }

    shortest.print_info();


}