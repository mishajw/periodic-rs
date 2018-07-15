extern crate periodic;

use std::thread;
use std::time::Duration;

fn main() {
    let mut planner = periodic::Planner::new();
    planner.start();

    // Run, and finish, after one second
    planner.add(
        || println!("after one second"),
        periodic::period::After::new(Duration::from_secs(1)),
    );
    thread::sleep(Duration::from_secs(2));

    // Planner still works
    planner.add(
        || println!("after one second"),
        periodic::period::After::new(Duration::from_secs(1)),
    );
    thread::sleep(Duration::from_secs(2));
}
