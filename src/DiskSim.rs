mod scheduling;
pub use self::scheduling::{
    fcfs::FCFS,
    sstf::SSTF,
    scan::SCAN,
    look::LOOK,
    request::Request,
    scheduler::{Scheduler, Direction},
    vec_owner::VecOwner
};

use std::{env, process::exit};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let mut args: Vec<String> = env::args().collect();
    dbg!(&mut args);

    if args.len() != 2 || args.len() == 3 {
        panic!("Incorrect argument amount");
    }
    
    let mut start: u16;
    let mut direction: &str;
    let mut requests: Vec<Request> = Vec::new();

    // Check if first argument is R
    if args[1] == "R" {
        // Generate random values
        println!("Random!");
    } else {
        start = match args[1].parse() {
            Ok(number) => number,
            Err(e) => panic!("{}", e),
        };

        direction = match args[2].as_str() {
            "H" => args[3].as_str(),
            "L" => args[3].as_str(),
            _ => panic!("Wrong direction!")
        };

        if args[3].starts_with("R") {
            args[3].remove(0);
            let num_req: u16 = match args[3].parse() {
                Ok(num_req) => num_req,
                Err(e) => panic!("{}", e)
            };

            for i in 0..num_req {
                let location: u16 = rng.gen_range(0..10000);
                requests.push(Request::new(location, i as u16));

            }
        } else {
            for i in 3..args.len() {
                let location: u16 =  match args[i].parse() {
                    Ok(location) => location,
                    Err(e) => panic!("{}", e)
                };

                requests.push(Request::new(location, (i-3) as u16));
            }
        }



    }

    // Otherwise parse:
    //  1. Starting location of disk head
    //  2. Starting direction
    //  3. Either R# or series of requests


    // TODO: Create each scheduler and add the requests to them

    exit(0);








    let mut first_come: FCFS = FCFS::new(3, 5)
        .with_direction("H");

    first_come.add(Request::new(4,0));
    first_come.add(Request::new(1,1));
    first_come.add(Request::new(5,2));
    first_come.add(Request::new(2,3));

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

    let mut shortest: SSTF = SSTF::new(3, 5)
        .with_direction("H");

    shortest.add(Request::new(4,0));
    shortest.add(Request::new(1,0));
    shortest.add(Request::new(5,0));
    shortest.add(Request::new(2,0));

    for _ in 0..shortest.length() {
        let next: Request = match shortest.next_request() {
            Some(next) => next,
            None => panic!("bro!"),
        };
        println!("{}", next);
    }

    shortest.print_info();


    let mut scan: SCAN = SCAN::new(3, 5)
        .with_direction("H");


    scan.add(Request::new(4,0));
    scan.add(Request::new(1,0));
    scan.add(Request::new(5,0));
    scan.add(Request::new(2,0));

    while scan.length() != 0 {
        let next: Request = match scan.next_request() {
            Some(next) => next,
            None => continue,
        };
        println!("{}", next);
    }

    scan.print_info();

    let mut c_scan: SCAN = SCAN::new(3, 5)
        .with_direction("H")
        .is_circular();


    c_scan.add(Request::new(4,0));
    c_scan.add(Request::new(1,0));
    c_scan.add(Request::new(5,0));
    c_scan.add(Request::new(2,0));

    while c_scan.length() != 0 {
        let next: Request = match c_scan.next_request() {
            Some(next) => next,
            None => continue,
        };
        println!("{}", next);
    }

    c_scan.print_info();


    let mut look: LOOK = LOOK::new(3, 5)
        .with_direction("H");


    look.add(Request::new(4,0));
    look.add(Request::new(1,0));
    look.add(Request::new(5,0));
    look.add(Request::new(2,0));

    while look.length() != 0 {
        let next: Request = match look.next_request() {
            Some(next) => next,
            None => continue,
        };
        println!("{}", next);
    }

    look.print_info();

    let mut c_look: LOOK = LOOK::new(3, 5)
        .with_direction("H")
        .is_circular();


    c_look.add(Request::new(4,0));
    c_look.add(Request::new(1,0));
    c_look.add(Request::new(5,0));
    c_look.add(Request::new(2,0));

    while c_look.length() != 0 {
        let next: Request = match c_look.next_request() {
            Some(next) => next,
            None => continue,
        };
        println!("{}", next);
    }

    c_look.print_info();
}