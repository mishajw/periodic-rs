//! Scheduling tool for running tasks at fixed intervals

#![warn(missing_docs)]

mod planner;
pub use planner::Planner;

pub mod period;

mod instant_iter;
