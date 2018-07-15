extern crate periodic;

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut planner = periodic::Planner::new();
    planner.start();

    // Run every second
    planner.add(
        || println!("every second"),
        periodic::period::Every::new(Duration::from_secs(1)),
    );

    // Run after three seconds
    planner.add(
        || println!("after three seconds"),
        periodic::period::After::new(Duration::from_secs(3)),
    );

    // Run at three, five, and seven seconds
    planner.add(
        || println!("after three, five, seven seconds"),
        vec![
            Duration::from_secs(3),
            Duration::from_secs(5),
            Duration::from_secs(7),
        ],
    );

    // Can also pass `std::time::Instant`s instead of `Duration`s
    planner.add(
        || println!("after three seconds"),
        Instant::now() + Duration::from_secs(3),
    );

    thread::sleep(Duration::from_secs(10));
}
