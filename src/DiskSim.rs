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

use std::env;
use rand::Rng;
use rand::seq::SliceRandom;

// The number of cylinders to be used in the simulation
const CYLINDERS: u16 = 9999;

fn main() {
    let mut rng = rand::thread_rng();

    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        panic!("Incorrect argument amount");
    }
    
    let start: u16;
    let direction: &str;
    let mut requests: Vec<Request> = Vec::new();

    // Check if first argument is R
    if args[1] == "R" {
        // Generate start
        start = rng.gen_range(0..=CYLINDERS);

        // Generate direction
        direction = ["L", "H"].choose(&mut rng).unwrap();

        // Generate requests
        for i in 0..100 {
            let location: u16 = rng.gen_range(0..(CYLINDERS+1));
            requests.push(Request::new(location, i as u16));
        }
    } else {
        // Otherwise parse:
        //  1. Starting location of disk head
        start = match args[1].parse() {
            Ok(number) => number,
            Err(e) => panic!("{}", e),
        };

        //  2. Starting direction
        direction = match args[2].as_str() {
            "H" => args[2].as_str(),
            "L" => args[2].as_str(),
            _ => panic!("Wrong direction!")
        };

        //  3. Either R# or series of requests
        if args[3].starts_with("R") {
            // args[3].remove(0);
            let num_rand = args[3].strip_prefix("R").unwrap();
            let num_req: u16 = match num_rand.parse() {
                Ok(num_req) => num_req,
                Err(e) => panic!("{}", e)
            };

            for i in 0..num_req {
                let location: u16 = rng.gen_range(0..(CYLINDERS+1));
                requests.push(Request::new(location, i as u16));
            }
        } else {
            // Parse through each request given
            for i in 3..args.len() {
                let location: u16 =  match args[i].parse() {
                    Ok(location) => location,
                    Err(e) => panic!("{}", e)
                };

                requests.push(Request::new(location, (i-3) as u16));
            }
        }
    }

    println!("== Service History ==");

    /* == Simulate First Come First Serve == */

    let mut first = FCFS::new(start, CYLINDERS)
        .with_direction(direction);

    first.add_vec(&requests);

    first.simulate_scheduling();

    /* == Simulate Shortest Seek Time First == */

    let mut short = SSTF::new(start, CYLINDERS)
        .with_direction(direction);

    short.add_vec(&requests);

    short.simulate_scheduling();

    /* == Simulate SCAN (Service back and forth) == */

    let mut scan = SCAN::new(start, CYLINDERS)
        .with_direction(direction);

    scan.add_vec(&requests);

    scan.remove_duplicates();

    scan.simulate_scheduling();

    /* == Simulate C-SCAN (Service back and forth, only servicing going HIGH) == */

    let mut c_scan = SCAN::new(start, CYLINDERS)
        .with_direction(direction)
        .is_circular();

    c_scan.add_vec(&requests);

    c_scan.remove_duplicates();

    c_scan.simulate_scheduling();

    /* == Simulate LOOK (Service going between lowest and highest request) == */

    let mut look = LOOK::new(start, CYLINDERS)
        .with_direction(direction);

    look.add_vec(&requests);

    look.remove_duplicates();

    look.simulate_scheduling();

    /* == Simulate C-LOOK (Service going between lowest and highest request, only servicing going HIGH) == */

    let mut c_look = LOOK::new(start, CYLINDERS)
        .with_direction(direction)
        .is_circular();

    c_look.add_vec(&requests);

    c_look.remove_duplicates();

    c_look.simulate_scheduling();


    println!("== Service Stats ==");

    first.print_info();
    short.print_info();
    scan.print_info();
    c_scan.print_info();
    look.print_info();
    c_look.print_info();
}