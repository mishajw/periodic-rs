//! Simple scheduling tool for running tasks at fixed intervals.
//!
//! Handles all threads for scheduling and running tasks. Note that callbacks
//! passed in must be able to execute asynchronously, and therefore require
//! traits `Fn` (not `FnMut`), `Sync`, and `Send`. They must also have a
//! `'static` lifetime.
//!
//! # Example usage
//! ```no_run
//! use std::time::Duration;
//!
//! let mut planner = periodic::Planner::new();
//! planner.add(
//!     || println!("every three seconds"),
//!     periodic::Every::new(Duration::from_secs(3)),
//! );
//! planner.start();
//! ```
//!
//! See `./examples` for more detailed usage.

#![warn(missing_docs)]

mod planner;
pub use planner::Planner;

mod period;
pub use period::{After, Every};

mod instant_iter;
